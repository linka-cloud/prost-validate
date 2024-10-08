mod bytes;
mod string;
pub mod utils;
mod errors;

pub use bytes::ValidateBytes;
pub use string::ValidateString;
pub use utils::VecExt;
pub use errors::*;

#[cfg(feature = "derive")]
pub use prost_validate_derive::Validator;

pub type Result<T, E = Error> = core::result::Result<T, E>;

pub trait Validator: Send + Sync {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}
