//!
//! The linker output.
//!

pub mod ignored;
pub mod linked;
pub mod unlinked;

use std::collections::BTreeMap;

use self::ignored::Ignored;
use self::linked::Linked;
use self::unlinked::Unlinked;

///
/// The linker output.
///
#[derive(Debug, Default, serde::Serialize)]
pub struct Output {
    /// Linked bytecode files with bytecode hashes.
    pub linked: BTreeMap<String, Linked>,
    /// Unlinked bytecode files with the list of unlinked symbols.
    pub unlinked: BTreeMap<String, Unlinked>,
    /// Ignored bytecode files that do not require linking.
    pub ignored: BTreeMap<String, Ignored>,
}
