use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("must be equal to {0:?}")]
    Const(Vec<u8>),
    #[error("bytes length must be equal to {0}")]
    Len(usize),
    #[error("bytes length must be greater than or equal to {0}")]
    MinLen(usize),
    #[error("bytes length must be less than or equal to {0}")]
    MaxLen(usize),
    #[error("must match pattern {0}")]
    Pattern(String),
    #[error("must have prefix {0:?}")]
    Prefix(Vec<u8>),
    #[error("must have suffix {0:?}")]
    Suffix(Vec<u8>),
    #[error("must contain {0:?}")]
    Contains(Vec<u8>),
    #[error("must be in {0:?}")]
    In(Vec<Vec<u8>>),
    #[error("must not be in {0:?}")]
    NotIn(Vec<Vec<u8>>),
    #[error("must be a valid IP address")]
    Ip,
    #[error("must be a valid IPv4 address")]
    Ipv4,
    #[error("must be a valid IPv6 address")]
    Ipv6,
}

impl From<Error> for super::Error {
    fn from(value: Error) -> Self {
        Self::Bytes(value)
    }
}
