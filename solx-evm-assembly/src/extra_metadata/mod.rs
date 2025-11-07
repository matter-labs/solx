//!
//! The `solc --standard-json` output contract EVM extra metadata.
//!

pub mod defined_function;

use self::defined_function::DefinedFunction;

///
/// The `solc --standard-json` output contract EVM extra metadata.
///
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExtraMetadata {
    /// The list of defined functions.
    #[serde(default, rename = "recursiveFunctions")]
    pub defined_functions: Vec<DefinedFunction>,
}

impl ExtraMetadata {
    ///
    /// Returns the function reference for the specified tag.
    ///
    pub fn get(
        &self,
        code_segment: solx_utils::CodeSegment,
        tag: &num::BigUint,
    ) -> Option<&DefinedFunction> {
        for function in self.defined_functions.iter() {
            match code_segment {
                solx_utils::CodeSegment::Deploy => {
                    if function
                        .creation_tag
                        .map(|creation_tag| &num::BigUint::from(creation_tag) == tag)
                        .unwrap_or_default()
                    {
                        return Some(function);
                    }
                }
                solx_utils::CodeSegment::Runtime => {
                    if function
                        .runtime_tag
                        .map(|runtime_tag| &num::BigUint::from(runtime_tag) == tag)
                        .unwrap_or_default()
                    {
                        return Some(function);
                    }
                }
            }
        }

        None
    }
}
