//!
//! The lexical token location.
//!

use std::fmt;

///
/// The token location in the source code file.
///
#[derive(Debug, Default, Clone, Copy, Eq)]
pub struct Location {
    /// The line number, starting from 1.
    pub line: usize,
    /// The column number, starting from 1.
    pub column: usize,
}

impl Location {
    ///
    /// Creates a location with a file identifier.
    /// The file identifier can be used to get its contents from the global index.
    ///
    pub fn new() -> Self {
        Self { line: 1, column: 1 }
    }

    ///
    /// Creates a location by shifting the original one down by `lines` and
    /// setting the column to `column`. If the `lines` equals zero, shift the original column by `size`.
    ///
    pub fn shifted_down(&self, lines: usize, column: usize, size: usize) -> Self {
        if lines == 0 {
            Self {
                line: self.line,
                column: self.column + size,
            }
        } else {
            Self {
                line: self.line + lines,
                column,
            }
        }
    }

    ///
    /// Creates a location by shifting the original one rightward by `columns`.
    ///
    pub fn shifted_right(&self, columns: usize) -> Self {
        Self {
            line: self.line,
            column: self.column + columns,
        }
    }

    ///
    /// Creates a location for testing purposes.
    ///
    /// If the `file_index` feature is enabled, fetches the current file index
    /// from the global storage.
    ///
    pub fn test(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line && self.column == other.column
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.line, self.column) {
            (0, 0) => write!(f, "<unavailable>"),
            (line, column) => {
                write!(f, "{line}:{column}")
            }
        }
    }
}
