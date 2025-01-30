//!
//! CLI tests for the eponymous option.
//!

use predicates::prelude::*;
use tempfile::TempDir;
use test_case::test_case;

#[test]
fn default() -> anyhow::Result<()> {
    crate::common::setup()?;

    let tmp_dir_solx = TempDir::with_prefix("solx_output")?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--output-dir",
        tmp_dir_solx.path().to_str().unwrap(),
    ];

    let result = crate::cli::execute_solx(args)?;
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"));

    assert!(tmp_dir_solx.path().exists());

    Ok(())
}

// TODO: fix when Yul is fixed
// #[test_case(era_compiler_common::EXTENSION_EVM_BINARY)]
// fn yul(extension: &str) -> anyhow::Result<()> {
//     crate::common::setup()?;

//     let tmp_dir_solx = TempDir::with_prefix("solx_output")?;

//     let input_path = PathBuf::from(crate::common::TEST_YUL_CONTRACT_PATH);
//     let input_file = input_path
//         .file_name()
//         .expect("Always exists")
//         .to_str()
//         .expect("Always valid");

//     let args = &[
//         input_path.to_str().expect("Always valid"),
//         "--yul",
//         "--bin",
//         "--output-dir",
//         tmp_dir_solx.path().to_str().unwrap(),
//     ];

//     let result = crate::cli::execute_solx(args)?;
//     result
//         .success()
//         .stderr(predicate::str::contains("Compiler run successful"));

//     let output_file = tmp_dir_solx
//         .path()
//         .join(input_file)
//         .join(format!("Test.{extension}"));
//     assert!(output_file.exists());

//     Ok(())
// }

#[test_case(crate::common::SOLIDITY_ASM_OUTPUT_NAME)]
fn asm_and_metadata(asm_file_name: &str) -> anyhow::Result<()> {
    crate::common::setup()?;

    let tmp_dir_solx = TempDir::with_prefix("solx_output")?;

    let mut asm_path = tmp_dir_solx.path().to_path_buf();
    asm_path.push(crate::common::TEST_SOLIDITY_CONTRACT_NAME);
    asm_path.push(asm_file_name);

    let mut metadata_path = tmp_dir_solx.path().to_path_buf();
    metadata_path.push(crate::common::TEST_SOLIDITY_CONTRACT_NAME);
    metadata_path.push("Test_meta.json");

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--asm",
        "--metadata",
        "--output-dir",
        tmp_dir_solx.path().to_str().unwrap(),
    ];

    let result = crate::cli::execute_solx(args)?;
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"));

    assert!(tmp_dir_solx.path().exists());

    assert!(asm_path.exists());
    assert!(metadata_path.exists());

    Ok(())
}

#[test]
fn unusual_path_characters() -> anyhow::Result<()> {
    crate::common::setup()?;

    let tmp_dir_solx = TempDir::with_prefix("File!and#$%-XXXXXX")?;

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--bin",
        "--output-dir",
        tmp_dir_solx.path().to_str().unwrap(),
    ];

    let result = crate::cli::execute_solx(args)?;
    result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"));

    assert!(tmp_dir_solx.path().exists());

    Ok(())
}

#[test]
fn standard_json() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--output-dir",
        "output",
    ];

    let result = crate::cli::execute_solx(args)?;
    result.success().stdout(predicate::str::contains(
        "Output directory cannot be used in standard JSON mode.",
    ));

    Ok(())
}
