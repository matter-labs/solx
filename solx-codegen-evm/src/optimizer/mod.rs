//!
//! The LLVM optimizing tools.
//!

pub mod settings;

use crate::target_machine::TargetMachine;

use self::settings::Settings;

///
/// The LLVM optimizing tools.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Optimizer {
    /// The optimizer settings.
    settings: Settings,
}

impl Optimizer {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(settings: Settings) -> Self {
        Self { settings }
    }

    ///
    /// Runs the new pass manager.
    ///
    pub fn run(
        &self,
        target_machine: &TargetMachine,
        module: &inkwell::module::Module,
    ) -> Result<(), inkwell::support::LLVMString> {
        target_machine.run_optimization_passes(
            module,
            format!("default<O{}>", self.settings.middle_end_as_char()).as_str(),
        )
    }

    ///
    /// Returns the optimizer settings reference.
    ///
    pub fn settings(&self) -> &Settings {
        &self.settings
    }
}
