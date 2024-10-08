use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("must be equal to {0:?}")]
    Const(i32),
    #[error("must be only one of the specified values")]
    DefinedOnly,
    #[error("must be in {0:?}")]
    In(Vec<i32>),
    #[error("must not be in {0:?}")]
    NotIn(Vec<i32>),
}

impl From<Error> for super::Error {
    fn from(value: Error) -> Self {
        Self::Enum(value)
    }
}
