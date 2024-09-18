use std::borrow::Cow;
use prost_reflect::Value;
use time::{Duration as TimeDelta, OffsetDateTime};
use prost_types::{Duration, Timestamp};

pub(crate) trait AsDateTime {
    fn as_datetime(&self) -> OffsetDateTime;
}

impl AsDateTime for Timestamp {
    fn as_datetime(&self) -> OffsetDateTime {
        OffsetDateTime::from_unix_timestamp(self.seconds).unwrap_or(OffsetDateTime::from_unix_timestamp(0).unwrap()) + TimeDelta::nanoseconds(self.nanos as i64)
    }
}

pub(crate) trait AsDuration {
    fn as_duration(self) -> TimeDelta;
}

impl AsDuration for Duration {
    fn as_duration(self) -> TimeDelta {
        TimeDelta::new(self.seconds, self.nanos)
    }
}

impl AsDuration for Option<Duration> {
    fn as_duration(self) -> TimeDelta{
        self.map(|d| d.as_duration()).unwrap_or(TimeDelta::default())
    }
}

pub(crate) fn is_set(val: &Cow<Value>) -> bool {
    match val {
        Cow::Borrowed(_) => true,
        Cow::Owned(_) => false,
    }
}
