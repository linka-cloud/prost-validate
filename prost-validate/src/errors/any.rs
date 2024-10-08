use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("required")]
    Required,
    #[error("type_url must be in {0:?}")]
    In(Vec<String>),
    #[error("type_url must not be in {0:?}")]
    NotIn(Vec<String>),
}

impl From<Error> for super::Error {
    fn from(value: Error) -> Self {
        Self::Any(value)
    }
}
