use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("must be equal to {0:?}")]
    Const(time::OffsetDateTime),
    #[error("must be less than {0:?}")]
    Lt(time::OffsetDateTime),
    #[error("must be less than or equal to {0:?}")]
    Lte(time::OffsetDateTime),
    #[error("must be greater than {0:?}")]
    Gt(time::OffsetDateTime),
    #[error("must be greater than or equal to {0:?}")]
    Gte(time::OffsetDateTime),
    #[error("must be in range {0}{1:?}, {2:?}{3}")]
    InRange(String, time::OffsetDateTime, time::OffsetDateTime, String),
    #[error("must not be in range {0}{1:?}, {2:?}{3}")]
    NotInRange(String, time::OffsetDateTime, time::OffsetDateTime, String),
    #[error("must less than current time")]
    LtNow,
    #[error("must be less than now or within {0} from now")]
    LtNowWithin(time::Duration),
    #[error("must be greater than current time")]
    GtNow,
    #[error("must be greater than now or within {0} from now")]
    GtNowWithin(time::Duration),
    #[error("must be within {0:?} from current time")]
    Within(time::Duration),
}

impl Error {
    pub fn in_range(
        start_inclusive: bool,
        start: time::OffsetDateTime,
        end: time::OffsetDateTime,
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
        start: time::OffsetDateTime,
        end: time::OffsetDateTime,
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
        Self::Timestamp(value)
    }
}
