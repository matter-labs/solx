//!
//! The Ethereal IR block visited element.
//!

use std::cmp::Ordering;

///
/// The Ethereal IR block visited element.
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VisitedElement {
    /// The block key.
    pub block_key: era_compiler_llvm_context::BlockKey,
    /// The initial stack state hash.
    pub stack_hash: [u8; era_compiler_common::BYTE_LENGTH_FIELD],
}

impl VisitedElement {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        block_key: era_compiler_llvm_context::BlockKey,
        stack_hash: [u8; era_compiler_common::BYTE_LENGTH_FIELD],
    ) -> Self {
        Self {
            block_key,
            stack_hash,
        }
    }
}

impl PartialOrd for VisitedElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for VisitedElement {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.block_key.code_segment, other.block_key.code_segment) {
            (
                era_compiler_common::CodeSegment::Deploy,
                era_compiler_common::CodeSegment::Runtime,
            ) => Ordering::Less,
            (
                era_compiler_common::CodeSegment::Runtime,
                era_compiler_common::CodeSegment::Deploy,
            ) => Ordering::Greater,
            (
                era_compiler_common::CodeSegment::Deploy,
                era_compiler_common::CodeSegment::Deploy,
            )
            | (
                era_compiler_common::CodeSegment::Runtime,
                era_compiler_common::CodeSegment::Runtime,
            ) => {
                let tag_comparison = self.block_key.tag.cmp(&other.block_key.tag);
                if tag_comparison == Ordering::Equal {
                    if self.stack_hash == other.stack_hash {
                        Ordering::Equal
                    } else {
                        Ordering::Less
                    }
                } else {
                    tag_comparison
                }
            }
        }
    }
}
