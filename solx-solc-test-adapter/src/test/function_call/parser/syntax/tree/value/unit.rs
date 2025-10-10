//!
//! The unit.
//!

///
/// The unit.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unit {
    /// `ether` in the source code.
    Ether,
    /// `wei` in the source code.
    Wei,
}

impl Unit {
    ///
    /// A shortcut constructor.
    ///
    pub fn ether() -> Self {
        Self::Ether
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn wei() -> Self {
        Self::Wei
    }
}
