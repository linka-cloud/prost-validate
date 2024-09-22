use crate::validate::VALIDATION_FIELD_RULES;
use crate::validate_proto::FieldRules;
use prost_reflect::{FieldDescriptor, Value};
use prost_types::{Duration, Timestamp};
use std::borrow::Cow;
use std::sync::Arc;
use time::{Duration as TimeDelta, OffsetDateTime};

pub(crate) trait AsDateTime {
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

pub(crate) trait AsDuration {
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

#[allow(clippy::ptr_arg)]
pub(crate) fn is_set(val: &Cow<Value>) -> bool {
    match val {
        Cow::Borrowed(_) => true,
        Cow::Owned(_) => false,
    }
}

pub(crate) fn get_field_rules(field: &FieldDescriptor) -> anyhow::Result<Option<Arc<FieldRules>>> {
    let opts = field.options();
    let rules = opts.get_extension(&VALIDATION_FIELD_RULES);
    let rules = match rules.as_message() {
        Some(r) => r,
        None => return Ok(None),
    };
    Ok(Some(Arc::new(rules.transcode_to::<FieldRules>()?)))
}
