//!
//! Common interface for different compiler modes.
//!

///
/// Common interface for different compiler modes.
///
pub trait IMode {
    /// Optimization level, if applicable.
    fn optimizations(&self) -> Option<String>;

    /// Codegen version, if applicable.
    fn codegen(&self) -> Option<String>;

    /// Language version, if applicable.
    fn version(&self) -> Option<String>;
}

pub fn mode_to_string_aux(mode: &impl IMode, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for (index, element) in [mode.codegen(), mode.optimizations(), mode.version()]
        .iter()
        .flatten()
        .enumerate()
    {
        if index > 0 {
            write!(f, " ")?;
        }
        write!(f, "{element}")?;
    }
    Ok(())
}
