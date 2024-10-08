mod bytes;
mod error;
pub mod errors;
mod string;
#[doc(hidden)]
pub mod utils;

#[doc(hidden)]
pub use bytes::ValidateBytesExt;
pub use error::*;
#[doc(hidden)]
pub use string::ValidateStringExt;
#[doc(hidden)]
pub use utils::VecExt;

/// Re-export of the `Validator` derive macro if the `derive` feature is enabled.
#[cfg(feature = "derive")]
pub use prost_validate_derive::Validator;

/// A type alias for `Result` with the error type defaulting to `Error`.
pub type Result<T = (), E = Error> = core::result::Result<T, E>;

/// The trait implemented by types that require validation logic.
pub trait Validator: Send + Sync {
    fn validate(&self) -> Result {
        Ok(())
    }
}
