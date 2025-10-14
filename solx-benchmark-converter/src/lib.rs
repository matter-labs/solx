//!
//! The benchmark analyzer library.
//!

#![allow(clippy::large_enum_variant)]
#![allow(clippy::let_and_return)]

pub mod input;
pub mod model;
pub mod output;
pub mod results;

pub use crate::input::error::Error as InputReportError;
pub use crate::input::foundry_gas::FoundryGasReport;
pub use crate::input::source::Source as InputSource;
pub use crate::input::Input as InputReport;
pub use crate::model::benchmark::test::input::Input;
pub use crate::model::benchmark::test::metadata::Metadata as TestMetadata;
pub use crate::model::benchmark::test::selector::Selector as TestSelector;
pub use crate::model::benchmark::test::Test;
pub use crate::model::benchmark::Benchmark;
pub use crate::model::evm_interpreter::GROUP_NAME as TEST_GROUP_EVM_INTERPRETER;
pub use crate::output::format::Format as OutputFormat;
pub use crate::output::Output;
