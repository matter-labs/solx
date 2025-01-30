//!
//! Integer counter starting from zero. Supports increment, reset and getting
//! the current value.
//!

type IntType = u32;

///
/// Integer counter starting from zero. Supports increment, reset and getting
/// the current value.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Counter {
    value: IntType,
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

impl Counter {
    ///
    /// Returns a new, zero-initialized counter.
    ///
    pub fn new() -> Self {
        Self { value: 0 }
    }

    ///
    /// Get the current value of the counter.
    ///
    pub fn get_value(&self) -> IntType {
        self.value
    }

    ///
    /// Increment the counter.
    ///
    pub fn increment(&mut self) {
        self.value += 1
    }

    ///
    /// Reset counter to zero..
    ///
    pub fn reset(&mut self) {
        self.value = 0
    }
}

impl From<Counter> for IntType {
    fn from(value: Counter) -> Self {
        value.get_value()
    }
}
