//!
//! The compiler cache value.
//!

use std::sync::Arc;
use std::sync::Mutex;

///
/// The compiler cache value.
///
pub enum Value<T> {
    /// The value is being evaluated.
    Waiter(Arc<Mutex<()>>),
    /// The value is already evaluated.
    Value(T),
}

impl<T> Value<T> {
    ///
    /// A shortcut waiter constructor.
    ///
    pub fn waiter() -> Arc<Mutex<()>> {
        Arc::new(Mutex::new(()))
    }

    ///
    /// Unwraps the value and returns a reference.
    ///
    /// # Panics
    ///
    /// If the value is evaluated.
    ///
    pub fn unwrap_value(&self) -> &T {
        match self {
            Self::Value(value) => value,
            _ => panic!("Value is not evaluated"),
        }
    }
}
