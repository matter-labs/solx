//!
//! CLI tests for the eponymous option.
//!

use era_compiler_common::EVMMetadataHashType;
use era_compiler_common::Target;
use predicates::prelude::*;
use test_case::test_case;

// #[test_case(EVMMetadataHashType::None.to_string())] TODO: move metadata to linker
// fn none(hash_type: era_compiler_common::EVMMetadataHashType) -> anyhow::Result<()> {
//     let _ = crate::common::setup();

//     let hash_type = hash_type.to_string();
//     let args = &[
//         crate::common::TEST_SOLIDITY_CONTRACT_PATH,
//         "--metadata-hash",
//         hash_type.as_str(),
//         "--no-cbor-metadata",
//         "--bin",
//     ];

//     let result = crate::cli::execute_solx(args)?;
//     result
//         .success()
//         .stdout(predicate::str::contains("Binary"))
//         .stdout(predicate::str::contains("a165").not())
//         .stdout(predicate::str::ends_with("0023").not());

//     Ok(())
// }

// #[test_case(EVMMetadataHashType::IPFS.to_string())] TODO: move metadata to linker
// fn ipfs_solidity(hash_type: era_compiler_common::EVMMetadataHashType) -> anyhow::Result<()> {
//     let _ = crate::common::setup();

//     let hash_type = hash_type.to_string();
//     let args = &[
//         crate::common::TEST_SOLIDITY_CONTRACT_PATH,
//         "--metadata-hash",
//         hash_type.as_str(),
//         "--no-cbor-metadata",
//         "--bin",
//     ];

//     let result = crate::cli::execute_solx(args)?;
//     result
//         .success()
//         .stdout(predicate::str::contains("Binary"))
//         .stdout(predicate::str::contains("a264").not())
//         .stdout(predicate::str::ends_with("0055").not());

//     Ok(())
// }

// #[test_case(EVMMetadataHashType::IPFS.to_string())] TODO: move metadata to linker
// fn ipfs_yul(hash_type: era_compiler_common::EVMMetadataHashType) -> anyhow::Result<()> {
//     let _ = crate::common::setup();

//     let hash_type = hash_type.to_string();
//     let args = &[
//         "--yul",
//         crate::common::TEST_YUL_CONTRACT_PATH,
//         "--metadata-hash",
//         hash_type.as_str(),
//         "--no-cbor-metadata",
//         "--bin",
//     ];

//     let result = crate::cli::execute_solx(args)?;
//     result
//         .success()
//         .stdout(predicate::str::contains("Binary"))
//         .stdout(predicate::str::contains("a264").not())
//         .stdout(predicate::str::ends_with("003e").not());

//     Ok(())
// }

// #[test_case(EVMMetadataHashType::IPFS.to_string())] TODO: move metadata to linker
// fn ipfs_llvm_ir(hash_type: era_compiler_common::EVMMetadataHashType) -> anyhow::Result<()> {
//     let _ = crate::common::setup();

//     let hash_type = hash_type.to_string();
//     let args = &[
//         "--llvm-ir",
//         crate::common::TEST_LLVM_IR_CONTRACT_PATH,
//         "--metadata-hash",
//         hash_type.as_str(),
//         "--no-cbor-metadata",
//         "--bin",
//     ];

//     let result = crate::cli::execute_solx(args)?;
//     result
//         .success()
//         .stdout(predicate::str::contains("Binary"))
//         .stdout(predicate::str::contains("a264").not())
//         .stdout(predicate::str::ends_with("003e").not());

//     Ok(())
// }

// #[test] TODO: move metadata to linker
// fn standard_json() -> anyhow::Result<()> {
//     crate::common::setup()?;

//     let args = &["--standard-json", crate::common::TEST_JSON_NO_CBOR_METADATA];

//     let result = crate::cli::execute_solx(args)?;
//     result
//         .success()
//         .stdout(predicate::str::contains("a264").not())
//         .stdout(predicate::str::ends_with("0055").not());

//     Ok(())
// }
