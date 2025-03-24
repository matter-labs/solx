//!
//! The Solidity compiler.
//!

#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::result_large_err)]

pub mod solc;
pub mod standard_json;
pub mod version;

pub use self::solc::Compiler;
pub use self::standard_json::input::language::Language as StandardJsonInputLanguage;
pub use self::standard_json::input::settings::libraries::Libraries as StandardJsonInputLibraries;
pub use self::standard_json::input::settings::metadata::Metadata as StandardJsonInputMetadata;
pub use self::standard_json::input::settings::optimizer::Optimizer as StandardJsonInputOptimizer;
pub use self::standard_json::input::settings::selection::selector::Selector as StandardJsonInputSelector;
pub use self::standard_json::input::settings::selection::Selection as StandardJsonInputSelection;
pub use self::standard_json::input::settings::Settings as StandardJsonInputSettings;
pub use self::standard_json::input::source::Source as StandardJsonInputSource;
pub use self::standard_json::input::Input as StandardJsonInput;
pub use self::standard_json::output::contract::evm::bytecode::Bytecode as StandardJsonOutputContractEVMBytecode;
pub use self::standard_json::output::contract::evm::extra_metadata::recursive_function::RecursiveFunction as StandardJsonOutputContractEVMExtraMetadataRecursiveFunction;
pub use self::standard_json::output::contract::evm::extra_metadata::ExtraMetadata as StandardJsonOutputContractEVMExtraMetadata;
pub use self::standard_json::output::contract::evm::EVM as StandardJsonOutputContractEVM;
pub use self::standard_json::output::contract::Contract as StandardJsonOutputContract;
pub use self::standard_json::output::error::collectable::Collectable as CollectableError;
pub use self::standard_json::output::error::source_location::SourceLocation as StandardJsonOutputErrorSourceLocation;
pub use self::standard_json::output::error::Error as StandardJsonOutputError;
pub use self::standard_json::output::Output as StandardJsonOutput;
pub use self::version::Version;

///
/// The compiler version default function.
///
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}
