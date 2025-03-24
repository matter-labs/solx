//!
//! The contract Yul source code.
//!

use std::collections::BTreeSet;

use solx_yul::yul::lexer::Lexer;
use solx_yul::yul::parser::statement::object::Object;

use crate::yul::parser::dialect::era::EraDialect;
use crate::yul::parser::wrapper::Wrap;

///
/// The contract Yul source code.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Yul {
    /// The Yul AST object.
    pub object: crate::yul::parser::statement::object::Object,
}

impl Yul {
    ///
    /// Transforms the `solc` standard JSON output contract into a Yul object.
    ///
    pub fn try_from_source(
        path: &str,
        source_code: &str,
        debug_config: Option<&era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<Option<Self>> {
        if source_code.is_empty() {
            return Ok(None);
        };

        if let Some(debug_config) = debug_config {
            debug_config.dump_yul(path, source_code)?;
        }

        let mut lexer = Lexer::new(source_code.to_owned());
        let object = Object::parse(&mut lexer, None)
            .map_err(|error| anyhow::anyhow!("Yul parsing: {error:?}"))?;

        Ok(Some(Self {
            object: object.wrap(),
        }))
    }

    ///
    /// Extracts the runtime code from the Yul object.
    ///
    pub fn take_runtime_code(&mut self) -> Option<Object<EraDialect>> {
        self.object.0.inner_object.take().map(|object| *object)
    }

    ///
    /// Get the list of unlinked deployable libraries.
    ///
    pub fn get_unlinked_libraries(&self) -> BTreeSet<String> {
        self.object.0.get_unlinked_libraries()
    }

    ///
    /// Get the list of EVM dependencies.
    ///
    pub fn get_evm_dependencies(
        &self,
        runtime_code: Option<&solx_yul::yul::parser::statement::object::Object<EraDialect>>,
    ) -> solx_yul::Dependencies {
        self.object.0.get_evm_dependencies(runtime_code)
    }
}

impl era_compiler_llvm_context::EVMWriteLLVM for Yul {
    fn declare(
        &mut self,
        context: &mut era_compiler_llvm_context::EVMContext,
    ) -> anyhow::Result<()> {
        self.object.declare(context)
    }

    fn into_llvm(self, context: &mut era_compiler_llvm_context::EVMContext) -> anyhow::Result<()> {
        self.object.into_llvm(context)
    }
}
