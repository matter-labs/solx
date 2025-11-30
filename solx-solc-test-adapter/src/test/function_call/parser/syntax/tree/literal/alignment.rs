//!
//! The alignment option.
//!

///
/// The alignment option.
///
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum Alignment {
    /// `{literal}` in the source code.
    #[default]
    Default,
    /// `left({literal})` in the source code.
    Left,
    /// `right({literal})` in the source code.
    Right,
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
