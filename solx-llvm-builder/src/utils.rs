//!
//! The LLVM builder utilities.
//!

use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;

use path_slash::PathBufExt;

/// The minimum required XCode version.
pub const XCODE_MIN_VERSION: u32 = 11;

/// The XCode version 15.
pub const XCODE_VERSION_15: u32 = 15;

///
/// The subprocess runner.
///
/// Checks the status and prints `stderr`.
///
pub fn command(command: &mut Command, description: &str) -> anyhow::Result<()> {
    if std::env::var("VERBOSE").is_ok() {
        println!("\ndescription: {description}; command: {command:?}");
    }
    if std::env::var("DRY_RUN").is_ok() {
        println!("\tOnly a dry run; not executing the command.");
    } else {
        let status = command
            .status()
            .map_err(|error| anyhow::anyhow!("{description} process: {error}"))?;
        if !status.success() {
            anyhow::bail!("{description} failed");
        }
    }
    Ok(())
}

///
/// Call ninja to build the LLVM.
///
pub fn ninja(build_dir: &Path) -> anyhow::Result<()> {
    let mut ninja = Command::new("ninja");
    ninja.args(["-C", build_dir.to_string_lossy().as_ref()]);
    if std::env::var("DRY_RUN").is_ok() {
        ninja.arg("-n");
    }
    command(ninja.arg("install"), "Running ninja install")?;
    Ok(())
}

///
/// Create an absolute path, appending it to the current working directory.
///
pub fn absolute_path<P: AsRef<Path>>(path: P) -> anyhow::Result<PathBuf> {
    let mut full_path = std::env::current_dir()?;
    full_path.push(path);
    Ok(full_path)
}

///
/// Converts a Windows path into a Unix path.
///
pub fn path_windows_to_unix<P: AsRef<Path> + PathBufExt>(path: P) -> anyhow::Result<PathBuf> {
    path.to_slash()
        .map(|pathbuf| PathBuf::from(pathbuf.to_string()))
        .ok_or_else(|| anyhow::anyhow!("Windows-to-Unix path conversion error"))
}

///
/// Checks if the tool exists in the system.
///
pub fn exists(name: &str) -> anyhow::Result<()> {
    let status = Command::new("which")
        .arg(name)
        .status()
        .map_err(|error| anyhow::anyhow!("`which {name}` process: {error}"))?;
    if !status.success() {
        anyhow::bail!("Tool `{name}` is missing. Please install");
    }
    Ok(())
}

///
/// Identify XCode version using `pkgutil`.
///
pub fn get_xcode_version() -> anyhow::Result<u32> {
    let pkgutil = Command::new("pkgutil")
        .args(["--pkg-info", "com.apple.pkg.CLTools_Executables"])
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|error| anyhow::anyhow!("`pkgutil` process: {error}"))?;
    let grep_version = Command::new("grep")
        .arg("version")
        .stdin(Stdio::from(pkgutil.stdout.expect(
            "Failed to identify XCode version - XCode or CLI tools are not installed",
        )))
        .output()
        .map_err(|error| anyhow::anyhow!("`grep` process: {error}"))?;
    let version_string = String::from_utf8(grep_version.stdout)?;
    let version_regex = regex::Regex::new(r"version: (\d+)\..*")?;
    let captures = version_regex
        .captures(version_string.as_str())
        .ok_or(anyhow::anyhow!(
            "Failed to parse XCode version: {version_string}"
        ))?;
    let xcode_version: u32 = captures
        .get(1)
        .expect("Always has a major version")
        .as_str()
        .parse()
        .map_err(|error| anyhow::anyhow!("Failed to parse XCode version: {error}"))?;
    Ok(xcode_version)
}
