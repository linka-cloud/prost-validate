use crate::make_error;
use thiserror::Error;

mod number;

pub mod any;
pub mod bool;
pub mod bytes;
pub mod duration;
pub mod r#enum;
pub mod list;
pub mod map;
pub mod message;
pub mod string;
pub mod timestamp;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("invalid validation rules: {0}")]
    InvalidRules(String),
    #[error(transparent)]
    Bool(bool::Error),
    #[error(transparent)]
    String(string::Error),
    #[error(transparent)]
    Bytes(bytes::Error),
    #[error(transparent)]
    Float(float::Error),
    #[error(transparent)]
    Double(double::Error),
    #[error(transparent)]
    Int32(int32::Error),
    #[error(transparent)]
    Int64(int64::Error),
    #[error(transparent)]
    Uint32(uint32::Error),
    #[error(transparent)]
    Uint64(uint64::Error),
    #[error(transparent)]
    Sint32(sint32::Error),
    #[error(transparent)]
    Sint64(sint64::Error),
    #[error(transparent)]
    Fixed32(fixed32::Error),
    #[error(transparent)]
    Fixed64(fixed64::Error),
    #[error(transparent)]
    Sfixed32(sfixed32::Error),
    #[error(transparent)]
    Sfixed64(sfixed64::Error),
    #[error(transparent)]
    List(list::Error),
    #[error(transparent)]
    Map(map::Error),
    #[error(transparent)]
    Duration(duration::Error),
    #[error(transparent)]
    Timestamp(timestamp::Error),
    #[error(transparent)]
    Message(message::Error),
    #[error(transparent)]
    Any(any::Error),
    #[error(transparent)]
    Enum(r#enum::Error),
}

make_error!(float, f32, Float);
make_error!(double, f64, Double);
make_error!(int32, i32, Int32);
make_error!(int64, i64, Int64);
make_error!(uint32, u32, Uint32);
make_error!(uint64, u64, Uint64);
make_error!(sint32, i32, Sint32);
make_error!(sint64, i64, Sint64);
make_error!(fixed32, u32, Fixed32);
make_error!(fixed64, u64, Fixed64);
make_error!(sfixed32, i32, Sfixed32);
make_error!(sfixed64, i64, Sfixed64);

// TODO(adphi): remove when not necessary anymore
impl From<&str> for Error {
    /// Converts a string into an `Error`.
    fn from(value: &str) -> Self {
        Self::InvalidRules(value.to_string())
    }
}

// TODO(adphi): remove when not necessary anymore
impl From<String> for Error {
    /// Converts a string into an `Error`.
    fn from(value: String) -> Self {
        Self::InvalidRules(value.to_string())
    }
}
