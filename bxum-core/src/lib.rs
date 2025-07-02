#![cfg_attr(test, allow(clippy::float_cmp))]
#![cfg_attr(not(test), warn(clippy::print_stdout, clippy::dbg_macro))]

// #[macro_use]
// pub(crate) mod macros;
#[doc(hidden)] // macro helpers
pub mod __private {
    #[cfg(feature = "tracing")]
    pub use tracing;
}

mod error;
pub use self::error::Error;


/// Alias for a type-erased error type.
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
