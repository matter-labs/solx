//!
//! The alignment option.
//!

///
/// The alignment option.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Alignment {
    /// `left({literal})` in the source code.
    Left,
    /// `right({literal})` in the source code.
    Right,
    /// `{literal}` in the source code.
    Default,
}

impl Default for Alignment {
    fn default() -> Self {
        Self::Default
    }
}

impl Alignment {
    ///
    /// A shortcut constructor.
    ///
    pub fn left() -> Self {
        Self::Left
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn right() -> Self {
        Self::Right
    }
}
