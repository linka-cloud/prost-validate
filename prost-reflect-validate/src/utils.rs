use chrono::{DateTime, TimeDelta, Utc};
use prost_types::{Duration, Timestamp};

pub(crate) trait AsDateTime {
    fn as_datetime(&self) -> DateTime<Utc>;
}

impl AsDateTime for Timestamp {
    fn as_datetime(&self) -> DateTime<Utc> {
        DateTime::from_timestamp(self.seconds, self.nanos as u32).unwrap()
    }
}

pub(crate) trait AsDuration {
    fn as_duration(&self) -> TimeDelta;
}

impl AsDuration for Duration {
    fn as_duration(&self) -> TimeDelta {
        TimeDelta::new(self.seconds, self.nanos as u32).unwrap()
    }
}

impl AsDuration for Option<Duration> {
    fn as_duration(&self) -> TimeDelta {
        match self {
            Some(d) => d.as_duration(),
            None => TimeDelta::zero(),
        }
    }
}
