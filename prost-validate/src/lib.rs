mod bytes;
mod string;
pub mod utils;

pub use bytes::ValidateBytes;
pub use string::ValidateString;
pub use utils::VecExt;

#[cfg(feature = "derive")]
pub use prost_validate_derive::Validator;

pub trait Validator: Send + Sync {
    fn validate(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
