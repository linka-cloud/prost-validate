use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("length must be equal to {0}")]
    MinPairs(usize),
    #[error("length must be equal to {0}")]
    MaxPairs(usize),
    #[error("key: {0}")]
    Keys(Box<crate::Error>),
    #[error("value: {0}")]
    Values(Box<crate::Error>),
    #[error("must no have sparse values")]
    NoSparse,
}

impl From<Error> for super::Error {
    fn from(value: Error) -> Self {
        Self::Map(value)
    }
}
