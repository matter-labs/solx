//!
//! The `solx` tester Solidity mode.
//!

use itertools::Itertools;

use crate::compilers::mode::imode::IMode;
use crate::compilers::mode::Mode as ModeWrapper;
use crate::compilers::solidity::codegen::Codegen;

///
/// The `solx` tester Solidity mode.
///
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Mode {
    /// The Solidity compiler version.
    pub solc_version: semver::Version,
    /// The Solidity compiler output type.
    pub solc_codegen: Codegen,
    /// Whether to enable the EVMLA codegen via Yul IR.
    pub via_ir: bool,
    /// Whether to enable the MLIR codegen.
    pub via_mlir: bool,
    /// Whether to run the Solidity compiler optimizer.
    pub solc_optimize: bool,
}

impl Mode {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        solc_version: semver::Version,
        solc_codegen: Codegen,
        via_ir: bool,
        via_mlir: bool,
        solc_optimize: bool,
    ) -> Self {
        Self {
            solc_version,
            solc_codegen,
            via_ir,
            via_mlir,
            solc_optimize,
        }
    }

    ///
    /// Unwrap mode.
    ///
    /// # Panics
    ///
    /// Will panic if the inner is non-Solidity mode.
    ///
    pub fn unwrap(mode: &ModeWrapper) -> &Self {
        match mode {
            ModeWrapper::Solc(mode) => mode,
            _ => panic!("Non-Solidity-upstream mode"),
        }
    }

    ///
    /// Checks if the mode is compatible with the source code pragmas.
    ///
    pub fn check_pragmas(&self, sources: &[(String, String)]) -> bool {
        sources.iter().all(|(_, source_code)| {
            match source_code.lines().find_map(|line| {
                let mut split = line.split_whitespace();
                if let (Some("pragma"), Some("solidity")) = (split.next(), split.next()) {
                    let version = split.join(",").replace(';', "");
                    semver::VersionReq::parse(version.as_str()).ok()
                } else {
                    None
                }
            }) {
                Some(pragma_version_req) => pragma_version_req.matches(&self.solc_version),
                None => true,
            }
        })
    }

    ///
    /// Checks if the mode is compatible with the Ethereum tests params.
    ///
    pub fn check_ethereum_tests_params(&self, params: &solx_solc_test_adapter::Params) -> bool {
        if !params.evm_version.matches_any(&[
            solx_solc_test_adapter::EVM::TangerineWhistle,
            solx_solc_test_adapter::EVM::SpuriousDragon,
            solx_solc_test_adapter::EVM::Byzantium,
            solx_solc_test_adapter::EVM::Constantinople,
            solx_solc_test_adapter::EVM::Petersburg,
            solx_solc_test_adapter::EVM::Istanbul,
            solx_solc_test_adapter::EVM::Berlin,
            solx_solc_test_adapter::EVM::London,
            solx_solc_test_adapter::EVM::Paris,
            solx_solc_test_adapter::EVM::Shanghai,
            solx_solc_test_adapter::EVM::Cancun,
        ]) {
            return false;
        }

        match self.solc_codegen {
            Codegen::Yul => {
                params.compile_via_yul != solx_solc_test_adapter::CompileViaYul::False
                    && params.abi_encoder_v1_only != solx_solc_test_adapter::ABIEncoderV1Only::True
            }
            Codegen::EVMLA if self.via_ir => {
                params.compile_via_yul != solx_solc_test_adapter::CompileViaYul::False
                    && params.abi_encoder_v1_only != solx_solc_test_adapter::ABIEncoderV1Only::True
            }
            Codegen::EVMLA => params.compile_via_yul != solx_solc_test_adapter::CompileViaYul::True,
        }
    }
}

impl IMode for Mode {
    fn optimizations(&self) -> Option<String> {
        Some((if self.solc_optimize { "+" } else { "-" }).to_string())
    }

    fn codegen(&self) -> Option<String> {
        Some(
            (if self.via_mlir {
                "L"
            } else {
                match self.solc_codegen {
                    Codegen::Yul => "Y",
                    Codegen::EVMLA if self.via_ir => "I",
                    Codegen::EVMLA => "E",
                }
            })
            .to_string(),
        )
    }

    fn version(&self) -> Option<String> {
        Some(self.solc_version.to_string())
    }
}
