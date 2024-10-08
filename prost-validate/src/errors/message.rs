use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("required")]
    Required,
    #[error("{0}")]
    Message(Box<crate::Error>),
}

impl From<Error> for super::Error {
    fn from(value: Error) -> Self {
        Self::Message(value)
    }
}
