//!
//! Unit test constants.
//!

/// A test input file.
pub const TEST_CONTRACTS_PATH: &str = "tests/data/contracts/";

/// A test input file.
pub const TEST_SOLIDITY_CONTRACT_NAME: &str = "Test.sol";

/// A test input file.
pub const TEST_SOLIDITY_CONTRACT_PATH: &str = "tests/data/contracts/solidity/Test.sol";

/// A test input file.
pub const TEST_SOLIDITY_CONTRACT_GREETER_PATH: &str = "tests/data/contracts/solidity/Greeter.sol";

/// A test input file.
pub const TEST_SOLIDITY_CONTRACT_CALLER_MAIN_PATH: &str =
    "tests/data/contracts/solidity/caller/Main.sol";

/// A test input file.
pub const TEST_SOLIDITY_CONTRACT_CALLER_CALLABLE_PATH: &str =
    "tests/data/contracts/solidity/caller/Callable.sol";

/// A test input file.
pub const TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH: &str =
    "tests/data/contracts/solidity/SimpleContract.sol";

/// A test input file.
pub const TEST_SOLIDITY_CONTRACT_LINKER_MIXED_DEPS_PATH: &str =
    "tests/data/contracts/solidity/LinkedMixedDeps.sol";

/// A test input file.
pub const TEST_SOLIDITY_CONTRACT_LINKER_MIXED_DEPS_MULTI_LEVEL_PATH: &str =
    "tests/data/contracts/solidity/LinkedMixedDepsMultiLevel.sol";

/// A test input file.
pub const TEST_SOLIDITY_CONTRACT_OPTIMIZED_PATH: &str =
    "tests/data/contracts/solidity/Optimized.sol";

/// A test input file.
pub const TEST_SOLIDITY_CONTRACT_INTERFACE_EMPTY_YUL_PATH: &str =
    "tests/data/contracts/solidity/InterfaceEmptyYul.sol";

/// A test input file.
pub const SOLIDITY_BIN_OUTPUT_NAME: &str = "Test.bin";

/// A test input file.
pub const TEST_YUL_CONTRACT_PATH: &str = "tests/data/contracts/yul/Test.yul";

/// A test input file.
pub const TEST_YUL_CONTRACT_OBJECT_NAMING_PATH: &str = "tests/data/contracts/yul/ObjectNaming.yul";

/// A test input file.
pub const TEST_LLVM_IR_CONTRACT_PATH: &str = "tests/data/contracts/llvm_ir/Test.ll";

/// A test input file.
pub const TEST_LLVM_IR_CONTRACT_INVALID_PATH: &str = "tests/data/contracts/llvm_ir/Invalid.ll";

/// A test input file.
pub const TEST_LLVM_IR_CONTRACT_LINKER_ERROR_PATH: &str =
    "tests/data/contracts/llvm_ir/LinkerError.ll";

/// A test input file.
pub const TEST_SOLIDITY_STANDARD_JSON_NON_EXISTENT_PATH: &str =
    "tests/data/standard_json_input/non_existent.json";

/// A test input file.
pub const TEST_SOLIDITY_STANDARD_JSON_INVALID_UTF8_PATH: &str =
    "tests/data/standard_json_input/invalid_utf8.json";

/// A test input file.
pub const TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH: &str =
    "tests/data/standard_json_input/solidity_solc.json";

/// A test input file.
pub const TEST_SOLIDITY_STANDARD_JSON_DEPLOY_TIME_LINKING_PATH: &str =
    "tests/data/standard_json_input/solidity_deploy_time_linking.json";

/// A test input file.
pub const TEST_SOLIDITY_STANDARD_JSON_SOLC_EMPTY_SOURCES_PATH: &str =
    "tests/data/standard_json_input/solidity_solc_empty_sources.json";

/// A test input file.
pub const TEST_SOLIDITY_STANDARD_JSON_SOLC_MISSING_SOURCES_PATH: &str =
    "tests/data/standard_json_input/solidity_solc_missing_sources.json";

/// A test input file.
pub const TEST_SOLIDITY_STANDARD_JSON_SOLC_INVALID_PATH: &str =
    "tests/data/standard_json_input/solidity_solc_invalid.json";

/// A test input file.
pub const TEST_SOLIDITY_STANDARD_JSON_SOLX_RECURSION_PATH: &str =
    "tests/data/standard_json_input/solidity_solx_recursion.json";

/// A test input file.
pub const TEST_SOLIDITY_STANDARD_JSON_SOLX_INVALID_PATH: &str =
    "tests/data/standard_json_input/solidity_solx_invalid.json";

/// A test input file.
pub const TEST_YUL_STANDARD_JSON_SOLC_PATH: &str = "tests/data/standard_json_input/yul_solc.json";

/// A test input file.
pub const TEST_YUL_STANDARD_JSON_SOLC_INVALID_PATH: &str =
    "tests/data/standard_json_input/yul_solc_urls_invalid.json";

/// A test input file.
pub const TEST_LLVM_IR_STANDARD_JSON_PATH: &str =
    "tests/data/standard_json_input/llvm_ir_urls.json";

/// A test input file.
pub const TEST_LLVM_IR_STANDARD_JSON_INVALID_PATH: &str =
    "tests/data/standard_json_input/llvm_ir_urls_invalid.json";

/// A test input file.
pub const TEST_LLVM_IR_STANDARD_JSON_MISSING_FILE_PATH: &str =
    "tests/data/standard_json_input/llvm_ir_urls_missing_file.json";

/// A test input file.
pub const TEST_JSON_METADATA_HASH_IPFS_AND_METADATA: &str =
    "tests/data/standard_json_input/metadata_hash_ipfs_and_metadata.json";

/// A test input file.
pub const TEST_JSON_METADATA_HASH_IPFS_NO_METADATA: &str =
    "tests/data/standard_json_input/metadata_hash_ipfs_no_metadata.json";

/// A test input file.
pub const TEST_JSON_METADATA_HASH_NONE_AND_METADATA: &str =
    "tests/data/standard_json_input/metadata_hash_none_and_metadata.json";

/// A test input file.
pub const TEST_JSON_METADATA_HASH_NONE_NO_METADATA: &str =
    "tests/data/standard_json_input/metadata_hash_none_no_metadata.json";

/// A test input file.
pub const TEST_JSON_NO_CBOR_METADATA: &str = "tests/data/standard_json_input/no_cbor_metadata.json";

/// The broken input file path.
pub const TEST_BROKEN_INPUT_PATH: &str = "tests/data/broken.bad";

/// A non-existent path.
pub const TEST_NON_EXISTENT_PATH: &str = "tests/data/non_existent";

/// A test constant.
pub const LIBRARY_DEFAULT: &str = "tests/data/contracts/solidity/MiniMath.sol:MiniMath=0xF9702469Dfb84A9aC171E284F71615bd3D3f1EdC";

/// A test constant.
pub const LIBRARY_CONTRACT_NAME_MISSING: &str =
    "tests/data/contracts/solidity/MiniMath.sol=0xF9702469Dfb84A9aC171E284F71615bd3D3f1EdC";

/// A test constant.
pub const LIBRARY_ADDRESS_MISSING: &str = "tests/data/contracts/solidity/MiniMath.sol:MiniMath";

/// A test constant.
pub const LIBRARY_ADDRESS_INVALID: &str =
    "tests/data/contracts/solidity/MiniMath.sol:MiniMath=INVALID";

/// A test constant.
pub const LIBRARY_LINKER: &str =
    "Greeter.sol:GreeterHelper=0x1234567890abcdef1234567890abcdef12345678";

/// A test constant.
pub const LIBRARY_LINKER_CONTRACT_NAME_MISSING: &str =
    "Greeter.sol=0x1234567890abcdef1234567890abcdef12345678";

/// A test constant.
pub const LIBRARY_LINKER_ADDRESS_MISSING: &str = "Greeter.sol:GreeterHelper";

/// A test constant.
pub const LIBRARY_LINKER_ADDRESS_INVALID: &str = "Greeter.sol:GreeterHelper=XINVALID";

/// A test constant.
pub const LIBRARY_LINKER_ADDRESS_INCORRECT_SIZE: &str = "Greeter.sol:GreeterHelper=0x12345678";
