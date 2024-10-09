use crate::errors;

/// Represents a validation error for a field.
#[derive(Debug, Clone)]
pub struct Error {
    /// The field associated with the error.
    pub field: String,
    /// The error message.
    pub details: errors::Error,
}

impl Error {
    /// Creates a new `Error` instance.
    #[allow(clippy::needless_pass_by_value)]
    pub fn new<T: ToString>(field: T, details: impl Into<errors::Error>) -> Self {
        Self {
            field: field.to_string(),
            details: details.into(),
        }
    }
}

impl std::fmt::Display for Error {
    /// Formats the error for display.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\"{}\": {}", self.field, self.details)
    }
}

#[cfg(feature = "tonic")]
impl From<Error> for tonic_types::FieldViolation {
    /// Converts an `Error` into a `FieldViolation`.
    fn from(value: Error) -> Self {
        Self {
            field: value.field,
            description: value.details.to_string(),
        }
    }
}

#[cfg(feature = "tonic")]
impl From<Error> for tonic_types::ErrorDetails {
    /// Converts an `Error` into `ErrorDetails`.
    fn from(value: Error) -> Self {
        tonic_types::ErrorDetails::with_bad_request(vec![value.into()])
    }
}

#[cfg(feature = "tonic")]
impl From<Error> for tonic::Status {
    /// Converts an `Error` into a `tonic::Status`.
    fn from(value: Error) -> Self {
        let code = match value.details {
            errors::Error::InvalidRules(_) => tonic::Code::Internal,
            _ => tonic::Code::InvalidArgument,
        };
        <tonic::Status as tonic_types::StatusExt>::with_error_details(
            code,
            value.to_string(),
            value.into(),
        )
    }
}

/// Macro to format an error.
///
/// # Arguments
///
/// * `$msg` - The error message.
/// * `$field` - The field associated with the error.
/// * `$arg` - Additional arguments for the error message.
///
/// # Returns
///
/// An `Error` instance.
#[macro_export]
macro_rules! format_err {
    ($msg:literal $(,)?) => {
        ::prost_validate::Error {
            field: "".to_string(),
            details: ::prost_validate::errors::Error::InvalidRules(format!("{}", $msg)),
        }
    };
    ($field:ident, $msg:ident) => {
        ::prost_validate::Error {
            field: format!("{}", $field),
            details: ::prost_validate::errors::Error::InvalidRules(format!("{}", $msg)),
        }
    };
    ($field:expr, $($arg:tt)*) => {
        ::prost_validate::Error {
            field: format!("{}", $field),
            details: ::prost_validate::errors::Error::InvalidRules(format!($($arg)*)),
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_format_err() {
        let err = format_err!("field", "is required");
        assert_eq!(err.to_string(), "\"field\": is required");

        let err = format_err!("field", "must be equal to {}", "value");
        assert_eq!(err.to_string(), "\"field\": must be equal to value");

        let field = "field";
        let err = format_err!(field, "is required");
        assert_eq!(err.to_string(), "\"field\": is required");

        let field = "field";
        let err = format_err!(field, "must be equal to {}", "value");
        assert_eq!(err.to_string(), "\"field\": must be equal to value");

        let field = "field";
        let err = format_err!(field, "{field}");
        assert_eq!(err.to_string(), "\"field\": field");
    }
}
