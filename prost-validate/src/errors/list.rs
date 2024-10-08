use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("must have at least {0} items")]
    MinItems(usize),
    #[error("must have at most {0} items")]
    MaxItems(usize),
    #[error("values must be unique")]
    Unique,
    #[error("{0}")]
    Item(Box<crate::Error>),
}

impl From<Error> for super::Error {
    fn from(value: Error) -> Self {
        Self::List(value)
    }
}
