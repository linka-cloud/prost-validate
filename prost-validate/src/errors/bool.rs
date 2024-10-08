use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("must be equal to {0}")]
    Const(bool),
}

impl From<Error> for super::Error {
    fn from(value: Error) -> Self {
        Self::Bool(value)
    }
}
