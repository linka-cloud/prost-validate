use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("must be equal to \"{0}\"")]
    Const(String),
    #[error("characters length must be equal to {0}")]
    Len(usize),
    #[error("characters length must be greater than or equal to {0}")]
    MinLen(usize),
    #[error("characters length must be less than or equal to {0}")]
    MaxLen(usize),
    #[error("bytes length must be equal to {0}")]
    LenBytes(usize),
    #[error("bytes length must be greater than or equal to {0}")]
    MinLenBytes(usize),
    #[error("bytes length must be less than or equal to {0}")]
    MaxLenBytes(usize),
    #[error("must match pattern \"{0}\"")]
    Pattern(String),
    #[error("must have prefix \"{0}\"")]
    Prefix(String),
    #[error("must have suffix \"{0}\"")]
    Suffix(String),
    #[error("must contain \"{0}\"")]
    Contains(String),
    #[error("must not contain \"{0}\"")]
    NotContains(String),
    #[error("must be in {0:?}")]
    In(Vec<String>),
    #[error("must not be in {0:?}")]
    NotIn(Vec<String>),
    #[error("must be a valid email address")]
    Email,
    #[error("must be a valid hostname")]
    Hostname,
    #[error("must be a valid IP address")]
    Ip,
    #[error("must be a valid IPv4 address")]
    Ipv4,
    #[error("must be a valid IPv6 address")]
    Ipv6,
    #[error("must be a valid URI")]
    Uri,
    #[error("must be a valid URI ref")]
    UriRef,
    #[error("must be a valid hostname or IP address")]
    Address,
    #[error("must be a valid UUID")]
    Uuid,
    #[error("must be a valid HTTP Header name")]
    HttpHeaderName,
    #[error("must be a valid HTTP Header value")]
    HttpHeaderValue,
}

impl From<Error> for super::Error {
    fn from(value: Error) -> Self {
        Self::String(value)
    }
}
