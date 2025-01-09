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

// NoopValidator is the same trait as `Validator`.
// It is used to implement the default behavior of a type that does not implement the `Validator` trait.
#[doc(hidden)]
pub trait NoopValidator {
    fn validate(&self) -> Result {
        Ok(())
    }
}

// Implement `NoopValidator` for any type.
impl<T: ?Sized> NoopValidator for T {}

// SafeValidator is a wrapper for any value.
// It is used to be able to call the validate method on any value.
#[doc(hidden)]
pub struct SafeValidator<'a, T: ?Sized>(pub &'a T);

// Implement the `validate` method only for types that implement the Validator trait.
impl<T: ?Sized + Validator> SafeValidator<'_, T> {
    pub fn validate(&self) -> Result {
        Validator::validate(self.0)
    }
}

/// Validate any value if it implements the Validator trait.
/// If the value does not implement the Validator trait, it will return Ok(()).
#[macro_export]
macro_rules! validate {
    ($value:tt) => {{
        use ::prost_validate::NoopValidator;
        use std::ops::Deref;
        ::prost_validate::SafeValidator($value.deref()).validate()
    }};
}

#[cfg(test)]
mod tests {
    pub struct A {}

    impl prost_validate::Validator for A {
        fn validate(&self) -> prost_validate::Result {
            Err(prost_validate::Error::new(
                "",
                prost_validate::errors::Error::InvalidRules("failed".to_string()),
            ))
        }
    }

    pub struct B {}

    #[test]
    fn test_validator() {
        let a = &A {};
        assert!(prost_validate::validate!(a).is_err());
    }
    #[test]
    fn test_validator_double_ref() {
        let a = &&A {};
        assert!(prost_validate::validate!(a).is_err());
    }
    #[test]
    fn test_non_validator() {
        let b = &B {};
        assert!(prost_validate::validate!(b).is_ok());
    }
    #[test]
    fn test_scalar() {
        let c = &42;
        assert!(prost_validate::validate!(c).is_ok());
    }
}
