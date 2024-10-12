use prost_types::{Duration, Timestamp};
use time::{Duration as TimeDelta, OffsetDateTime};

#[allow(clippy::unwrap_used)]
pub fn datetime(seconds: i64, nanos: i32) -> OffsetDateTime {
    OffsetDateTime::from_unix_timestamp(seconds)
        .unwrap_or(OffsetDateTime::from_unix_timestamp(0).unwrap())
        + TimeDelta::nanoseconds(nanos as i64)
}

pub fn duration(seconds: i64, nanos: i32) -> TimeDelta {
    TimeDelta::new(seconds, nanos)
}

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

impl AsDateTime for &Timestamp {
    fn as_datetime(&self) -> OffsetDateTime {
        (*self).as_datetime()
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
    #[allow(clippy::unwrap_used)]
    fn as_duration(&self) -> TimeDelta {
        self.map(|d| d.as_duration()).unwrap_or_default()
    }
}

pub trait VecExt<T> {
    fn unique(&self) -> Vec<T>
    where
        T: Clone + PartialEq;
}

macro_rules! unique {
    ($typ:ident) => {
        impl VecExt<$typ> for Vec<$typ> {
            fn unique(&self) -> Vec<$typ> {
                let mut seen = Vec::new();
                self.iter()
                    .filter(|x| {
                        if seen.contains(x) {
                            false
                        } else {
                            seen.push(x.clone());
                            true
                        }
                    })
                    .cloned()
                    .collect()
            }
        }
    };
}

macro_rules! unique_to_bits {
    ($typ:ident) => {
        impl VecExt<$typ> for Vec<$typ> {
            fn unique(&self) -> Vec<$typ> {
                let mut seen = Vec::new();
                self.iter()
                    .filter(|x| {
                        if seen.contains(&x.to_bits()) {
                            false
                        } else {
                            seen.push(x.to_bits());
                            true
                        }
                    })
                    .cloned()
                    .collect()
            }
        }
    };
}

unique!(String);
unique!(i32);
unique!(i64);
unique!(u32);
unique!(u64);
unique_to_bits!(f32);
unique_to_bits!(f64);

impl VecExt<Vec<u8>> for Vec<Vec<u8>> {
    fn unique(&self) -> Vec<Vec<u8>> {
        let mut seen = Vec::new();
        self.iter()
            .filter(|x| {
                if seen.contains(x) {
                    false
                } else {
                    #[allow(suspicious_double_ref_op)]
                    seen.push(x.clone());
                    true
                }
            })
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_float() {
        let vals = vec![1.0, 2.0, 1.0, 3.0, 2.0];
        let unique = vals.unique();
        assert_eq!(unique, vec![1.0, 2.0, 3.0]);
    }
}
