//!
//! The Ethereal IR of the EVM bytecode.
//!

pub mod entry_link;
pub mod function;

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;

use era_compiler_llvm_context::IContext;

use crate::assembly::instruction::Instruction;
use crate::extra_metadata::ExtraMetadata;

use self::function::block::Block;
use self::function::r#type::Type as FunctionType;
use self::function::Function;

///
/// Ethereal IR of EVM bytecode.
///
/// Ethereal IR (EthIR) is a special IR between the EVM legacy assembly and LLVM IR. It is
/// created to facilitate the translation and provide an additional environment for applying
/// transformations, duplication parts of the call and control flow graphs, tracking the
/// data flow, and a few more algorithms of static analysis.
///
/// The most important feature of EthIR is flattening the block tags and duplicating blocks for
/// each of initial states of the stack. LLVM IR supports only static control flow, so the
/// stack state must be known all the way throughout the program.
///
#[derive(Debug)]
pub struct EtherealIR {
    /// The all-inlined function.
    pub entry_function: Function,
    /// The recursive functions.
    pub recursive_functions: BTreeMap<era_compiler_llvm_context::BlockKey, Function>,
}

impl EtherealIR {
    /// The default entry function name.
    pub const DEFAULT_ENTRY_FUNCTION_NAME: &'static str = "main";

    /// The blocks hashmap initial capacity.
    pub const BLOCKS_HASHMAP_DEFAULT_CAPACITY: usize = 64;

    ///
    /// Assembles a sequence of functions from the sequence of instructions.
    ///
    pub fn new(
        solc_version: semver::Version,
        extra_metadata: ExtraMetadata,
        code_segment: Option<era_compiler_common::CodeSegment>,
        blocks: HashMap<era_compiler_llvm_context::BlockKey, Block>,
    ) -> anyhow::Result<Self> {
        let mut entry_function =
            Function::new(solc_version, code_segment, FunctionType::new_initial());
        let mut recursive_functions = BTreeMap::new();
        let mut visited_functions = BTreeSet::new();
        entry_function.traverse(
            &blocks,
            &mut recursive_functions,
            &extra_metadata,
            &mut visited_functions,
        )?;

        Ok(Self {
            entry_function,
            recursive_functions,
        })
    }

    ///
    /// Gets blocks for the specified type of the contract code.
    ///
    pub fn get_blocks(
        solc_version: semver::Version,
        code_segment: era_compiler_common::CodeSegment,
        instructions: &[Instruction],
    ) -> anyhow::Result<HashMap<era_compiler_llvm_context::BlockKey, Block>> {
        let mut blocks = HashMap::with_capacity(Self::BLOCKS_HASHMAP_DEFAULT_CAPACITY);
        let mut offset = 0;

        while offset < instructions.len() {
            let (block, size) = Block::try_from_instructions(
                solc_version.clone(),
                code_segment,
                &instructions[offset..],
            )?;
            blocks.insert(
                era_compiler_llvm_context::BlockKey::new(code_segment, block.key.tag.clone()),
                block,
            );
            offset += size;
        }

        Ok(blocks)
    }
}

impl era_compiler_llvm_context::EVMWriteLLVM for EtherealIR {
    fn declare(
        &mut self,
        context: &mut era_compiler_llvm_context::EVMContext,
    ) -> anyhow::Result<()> {
        self.entry_function.declare(context)?;

        for (_key, function) in self.recursive_functions.iter_mut() {
            function.declare(context)?;
        }

        Ok(())
    }

    fn into_llvm(self, context: &mut era_compiler_llvm_context::EVMContext) -> anyhow::Result<()> {
        context.evmla_mut().expect("Always exists").stack = vec![];

        self.entry_function.into_llvm(context)?;

        for (_key, function) in self.recursive_functions.into_iter() {
            function.into_llvm(context)?;
        }

        Ok(())
    }
}

impl std::fmt::Display for EtherealIR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.entry_function)?;

        for (_key, function) in self.recursive_functions.iter() {
            writeln!(f, "{function}")?;
        }

        Ok(())
    }
}
