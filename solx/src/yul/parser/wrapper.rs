//!
//! Proxy trait for `era_compiler_llvm_context::WriteLLVM`.
//!

///
/// Wraps a type in `Self::Wrapper`.
///
pub trait Wrap {
    /// Type to wrap in.
    type Wrapper;

    ///
    /// Wrap into an instance of [`Self::Wrapper`].
    ///
    fn wrap(self) -> Self::Wrapper;
}

///
/// Creates a wrapper structure for a Yul syntax tree node type
/// [`unwrapped_type`] and implements [`Wrap`] trait for it.
///
#[macro_export]
macro_rules! declare_wrapper {
    ($unwrapped_type:ty, $wrapped_type:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        #[doc = concat!("Wrapper for [`", stringify!($unwrapped_type), "`].")]
        pub struct $wrapped_type(pub $unwrapped_type);

        impl $crate::yul::parser::wrapper::Wrap for $unwrapped_type {
            type Wrapper = $wrapped_type;

            fn wrap(self) -> Self::Wrapper {
                $wrapped_type(self)
            }
        }
    };
}
