use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("must be equal to {0:?}")]
    Const(time::Duration),
    #[error("must be less than {0:?}")]
    Lt(time::Duration),
    #[error("must be less than or equal to {0:?}")]
    Lte(time::Duration),
    #[error("must be greater than {0:?}")]
    Gt(time::Duration),
    #[error("must be greater than or equal to {0:?}")]
    Gte(time::Duration),
    #[error("must be in range {0}{1:?}, {2:?}{3}")]
    InRange(String, time::Duration, time::Duration, String),
    #[error("must not be in range {0}{1:?}, {2:?}{3}")]
    NotInRange(String, time::Duration, time::Duration, String),
    #[error("must be in {0:?}")]
    In(Vec<time::Duration>),
    #[error("must not be in {0:?}")]
    NotIn(Vec<time::Duration>),
}

impl Error {
    pub fn in_range(
        start_inclusive: bool,
        start: time::Duration,
        end: time::Duration,
        end_inclusive: bool,
    ) -> Self {
        Self::InRange(
            if start_inclusive { "[" } else { "(" }.to_string(),
            start,
            end,
            if end_inclusive { "]" } else { ")" }.to_string(),
        )
    }
    pub fn not_in_range(
        start_inclusive: bool,
        start: time::Duration,
        end: time::Duration,
        end_inclusive: bool,
    ) -> Self {
        Self::NotInRange(
            if start_inclusive { "[" } else { "(" }.to_string(),
            start,
            end,
            if end_inclusive { "]" } else { ")" }.to_string(),
        )
    }
}

impl From<Error> for super::Error {
    fn from(value: Error) -> Self {
        Self::Duration(value)
    }
}
