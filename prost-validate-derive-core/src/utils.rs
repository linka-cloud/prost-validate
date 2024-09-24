use darling::FromMeta;
use prost_types::{Duration, Timestamp};
use time::{Duration as TimeDelta, OffsetDateTime};

pub trait AsDateTime {
    fn as_datetime(&self) -> OffsetDateTime;
}

impl AsDateTime for Timestamp {
    #[allow(clippy::unwrap_used)]
    fn as_datetime(&self) -> OffsetDateTime {
        OffsetDateTime::from_unix_timestamp(self.seconds)
            .unwrap_or(OffsetDateTime::from_unix_timestamp(0).unwrap())
            + TimeDelta::nanoseconds(self.nanos as i64)
    }
}

pub trait AsDuration {
    fn as_duration(&self) -> TimeDelta;
}

impl AsDuration for Duration {
    fn as_duration(&self) -> TimeDelta {
        TimeDelta::new(self.seconds, self.nanos)
    }
}

impl AsDuration for Option<Duration> {
    fn as_duration(&self) -> TimeDelta {
        self.map(|d| d.as_duration()).unwrap_or_default()
    }
}

pub trait IsTrueAnd<F, T>
where
    F: Fn() -> T,
{
    fn is_true_and(&self, f: F) -> Option<T>;
}

impl<F, T> IsTrueAnd<F, T> for Option<bool>
where
    F: Fn() -> T,
{
    fn is_true_and(&self, f: F) -> Option<T> {
        if self.unwrap_or_default() {
            Some(f())
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub enum StringOrBool {
    String(String),
    Bool(bool),
}

impl FromMeta for StringOrBool {
    fn from_word() -> darling::Result<Self> {
        Ok(Self::Bool(true))
    }
    fn from_string(value: &str) -> darling::Result<Self> {
        Ok(match value {
            "true" => Self::Bool(true),
            "false" => Self::Bool(false),
            _ => Self::String(value.to_string()),
        })
    }
}
