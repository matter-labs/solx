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
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Yul {
    /// Yul AST object.
    pub object: crate::yul::parser::statement::object::Object,
    /// Dependencies of the Yul object.
    pub dependencies: solx_yul::Dependencies,
    /// Runtime code object that is only set in deploy code.
    pub runtime_code: Option<Box<Self>>,
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

        let mut lexer = Lexer::new(source_code);
        let mut object = Object::parse(&mut lexer, None, era_compiler_common::CodeSegment::Deploy)
            .map_err(|error| anyhow::anyhow!("Yul parsing: {error:?}"))?;

        let runtime_code = object.inner_object.take().map(|object| {
            let dependencies = object.get_evm_dependencies(None);
            Self {
                object: object.wrap(),
                dependencies,
                runtime_code: None,
            }
        });
        let dependencies = object.get_evm_dependencies(
            runtime_code
                .as_ref()
                .map(|runtime_code| &runtime_code.object.0),
        );

        Ok(Some(Self {
            object: object.wrap(),
            dependencies,
            runtime_code: runtime_code.map(Box::new),
        }))
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
