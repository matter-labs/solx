//!
//! The contract LLVM IR source code.
//!

///
/// The contract LLVM IR source code.
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LLVMIR {
    /// LLVM IR file path.
    pub path: String,
    /// LLVM IR source code.
    pub source: String,
    /// Dependencies of the LLVM IR translation unit.
    pub dependencies: solx_yul::Dependencies,
}

impl LLVMIR {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        path: String,
        code_segment: era_compiler_common::CodeSegment,
        mut source: String,
    ) -> Self {
        source.push(char::from(0));

        let runtime_code_identifier =
            format!("{path}.{}", era_compiler_common::CodeSegment::Runtime);
        let dependencies = match code_segment {
            era_compiler_common::CodeSegment::Deploy => {
                let mut dependencies = solx_yul::Dependencies::new(path.as_str());
                dependencies.push(runtime_code_identifier.to_owned(), true);
                dependencies
            }
            era_compiler_common::CodeSegment::Runtime => {
                let dependencies = solx_yul::Dependencies::new(runtime_code_identifier.as_str());
                dependencies
            }
        };

        Self {
            path,
            source,
            dependencies,
        }
    }
}
