//!
//! The Solidity tests file system entity.
//!

pub mod changes;
pub mod directory;
pub mod enabled;
pub mod test_file;

use std::collections::BTreeMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use self::changes::Changes;
use self::directory::Directory;
use self::enabled::EnabledTest;
use self::test_file::TestFile;

///
/// The Solidity tests file system entity.
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FSEntity {
    /// The directory.
    Directory(Directory),
    /// The test file.
    File(TestFile),
}

impl FSEntity {
    ///
    /// Indexes the specified directory.
    ///
    pub fn index(path: &Path) -> anyhow::Result<FSEntity> {
        let mut entries = BTreeMap::new();

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            let entry_type = entry.file_type()?;

            if entry.file_name().to_string_lossy().starts_with('.') {
                continue;
            }

            if entry_type.is_dir() {
                entries.insert(
                    path.file_name()
                        .ok_or_else(|| anyhow::anyhow!("Failed to get filename"))?
                        .to_string_lossy()
                        .to_string(),
                    Self::index(&path)?,
                );
                continue;
            }

            if !entry_type.is_file() {
                anyhow::bail!("Invalid entry type");
            }

            entries.insert(
                path.file_name()
                    .ok_or_else(|| anyhow::anyhow!("Failed to get filename"))?
                    .to_string_lossy()
                    .to_string(),
                Self::File(TestFile::try_from(path.as_path())?),
            );
        }

        Ok(Self::Directory(Directory::new(entries)))
    }

    ///
    /// Updates the new index, tests and returns changes.
    ///
    pub fn update(
        &self,
        new: &mut FSEntity,
        initial: &Path,
        index_only: bool,
    ) -> anyhow::Result<Changes> {
        let mut changes = Changes::default();
        self.update_recursive(new, initial, &mut changes, index_only)?;
        Ok(changes)
    }

    ///
    /// Returns the enabled tests list with the `initial` directory prefix.
    ///
    pub fn into_enabled_list(self, initial: &Path) -> Vec<EnabledTest> {
        let mut accumulator = Vec::with_capacity(16384);
        self.into_enabled_list_recursive(initial, &mut accumulator);
        accumulator.sort_by_key(|test| test.path.to_owned());
        accumulator
    }

    ///
    /// Returns the enabled test by the path with the `initial` directory prefix (None if not found or test disabled).
    ///
    pub fn into_enabled_test(self, initial: &Path, path: &Path) -> Option<EnabledTest> {
        let mut current_entity = self;
        for path_part in path.iter() {
            match current_entity {
                FSEntity::Directory(mut directory) => {
                    current_entity = directory
                        .entries
                        .remove(path_part.to_string_lossy().as_ref())?;
                }
                FSEntity::File(_) => return None,
            }
        }
        match current_entity {
            FSEntity::Directory(_) => None,
            FSEntity::File(file) => {
                if !file.enabled {
                    return None;
                }
                let mut file_path = initial.to_path_buf();
                file_path.push(path);
                Some(EnabledTest::new(
                    file_path,
                    file.modes,
                    file.version,
                    file.group,
                ))
            }
        }
    }

    ///
    /// Updates new index, tests and lists changes.
    ///
    fn update_recursive(
        &self,
        new: &mut FSEntity,
        current: &Path,
        changes: &mut Changes,
        index_only: bool,
    ) -> anyhow::Result<()> {
        let (old_entities, new_entities) = match (self, new) {
            (Self::File(old_file), Self::File(new_file)) => {
                new_file.enabled = old_file.enabled;
                new_file.group = old_file.group.clone();
                new_file.comment = old_file.comment.clone();
                new_file.modes = old_file.modes.clone();
                new_file.version = old_file.version.clone();

                let new_hash = new_file
                    .hash
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("Test file hash is None: {current:?}"))?;

                if !old_file
                    .hash
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("Test file hash is None: {current:?}"))?
                    .eq(new_hash)
                {
                    if old_file.was_changed(current)? {
                        changes.conflicts.push(current.to_owned());
                    } else {
                        changes.updated.push(current.to_owned());
                    }
                    if !index_only {
                        TestFile::write_to_file(
                            current,
                            new_file
                                .data
                                .as_ref()
                                .ok_or_else(|| anyhow::anyhow!("Test data is None: {current:?}"))?
                                .as_bytes(),
                        )?;
                    }
                }
                return Ok(());
            }
            (
                Self::Directory(Directory {
                    enabled: old_enabled,
                    entries: old_entities,
                    comment: old_comment,
                }),
                Self::Directory(Directory {
                    enabled: new_enabled,
                    entries: new_entities,
                    comment: new_comment,
                }),
            ) => {
                *new_enabled = *old_enabled;
                *new_comment = old_comment.clone();

                (old_entities, new_entities)
            }
            (_, new) => {
                self.list_recursive(current, &mut changes.deleted);
                new.list_recursive(current, &mut changes.created);
                if !index_only {
                    self.delete(current)?;
                    new.create_recursive(current)?;
                }
                return Ok(());
            }
        };

        for (name, entity) in old_entities.iter() {
            let mut current = current.to_owned();
            current.push(name);
            if let Some(new_entity) = new_entities.get_mut(name) {
                entity.update_recursive(new_entity, &current, changes, index_only)?;
            } else {
                entity.list_recursive(&current, &mut changes.deleted);
                if !index_only {
                    entity.delete(&current)?;
                }
            }
        }
        for (name, entity) in new_entities.iter() {
            if !old_entities.contains_key(name) {
                let mut current = current.to_owned();
                current.push(name);
                entity.list_recursive(&current, &mut changes.created);
                if !index_only {
                    entity.create_recursive(&current)?;
                }
            }
        }

        Ok(())
    }

    ///
    /// Inner enabled accumulator function.
    ///
    fn into_enabled_list_recursive(self, current: &Path, accumulator: &mut Vec<EnabledTest>) {
        let entries = match self {
            Self::File(file) => {
                if !file.enabled {
                    return;
                }
                accumulator.push(EnabledTest::new(
                    current.to_owned(),
                    file.modes,
                    file.version,
                    file.group,
                ));
                return;
            }
            Self::Directory(directory) => {
                if !directory.enabled {
                    return;
                }
                directory.entries
            }
        };

        for (name, entity) in entries
            .into_iter()
            .filter(|(name, _entity)| !name.starts_with('_'))
        {
            let mut current = current.to_owned();
            current.push(name);
            entity.into_enabled_list_recursive(&current, accumulator);
        }
    }

    ///
    /// Creates files and folders from self.
    ///
    fn create_recursive(&self, current: &Path) -> anyhow::Result<()> {
        let entries = match self {
            Self::Directory(directory) => &directory.entries,
            Self::File(test) => {
                let mut file = File::create(current)
                    .map_err(|error| anyhow::anyhow!("Failed to create file: {error}"))?;
                file.write_all(
                    test.data
                        .as_ref()
                        .ok_or_else(|| anyhow::anyhow!("Test data is None: {current:?}"))?
                        .as_bytes(),
                )?;
                return Ok(());
            }
        };

        fs::create_dir(current)
            .map_err(|error| anyhow::anyhow!("Failed to create directory: {error}"))?;

        for (name, entity) in entries.iter() {
            let mut current = current.to_owned();
            current.push(name);
            entity.create_recursive(&current)?;
        }

        Ok(())
    }

    ///
    /// Deletes files and folders from self.
    ///
    fn delete(&self, current: &Path) -> anyhow::Result<()> {
        if let Self::Directory(_) = self {
            fs::remove_dir_all(current).map_err(|error| {
                anyhow::anyhow!("Failed to delete directory {current:?}: {error}")
            })?;
        } else {
            fs::remove_file(current)
                .map_err(|error| anyhow::anyhow!("Failed to delete file {current:?}: {error}"))?;
        }

        Ok(())
    }

    ///
    /// Inner accumulator function.
    ///
    fn list_recursive(&self, current: &Path, accumulator: &mut Vec<PathBuf>) {
        let entries = match self {
            Self::Directory(directory) => &directory.entries,
            Self::File(_) => {
                accumulator.push(current.to_owned());
                return;
            }
        };

        for (name, entity) in entries.iter() {
            let mut current = current.to_owned();
            current.push(name);
            entity.list_recursive(&current, accumulator);
        }
    }
}
