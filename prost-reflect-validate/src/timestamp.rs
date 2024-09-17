use crate::registry::FieldValidationFn;
use crate::utils::{AsDateTime, AsDuration};
use crate::validate_proto::field_rules::Type;
use crate::validate_proto::{FieldRules, TimestampRules};
use anyhow::{format_err};
use chrono::prelude::DateTime;
use chrono::{Timelike, Utc};
use prost_reflect::{DynamicMessage, FieldDescriptor};
use prost_types::Timestamp;
use std::sync::Arc;

fn push<F>(fns: &mut Vec<FieldValidationFn<Box<DynamicMessage>>>, name: Arc<String>, f: Arc<F>)
where
    F: Fn(&DateTime<Utc>, &TimestampRules, &String) -> anyhow::Result<bool> + Send + Sync + 'static,
{
    let name = name.clone();
    fns.push(Arc::new(move |val, rules| {
        let val = match val {
            Some(val) => val.transcode_to::<Timestamp>().unwrap().as_datetime(),
            _ => DateTime::from_timestamp(0, 0).unwrap(),
        };
        let rules = match rules.r#type {
            Some(Type::Timestamp(rules)) => rules,
            _ => return Err(format_err!("unexpected timestamp rules")),
        };
        f(&val, &rules, &name)
    }))
}

pub(crate) fn make_validate_timestamp(field: &FieldDescriptor, rules: &FieldRules) -> Vec<FieldValidationFn<Box<DynamicMessage>>> {
    let mut fns = Vec::new();
    let rules = match rules.r#type {
        Some(Type::Timestamp(rules)) => rules,
        _ => return fns,
    };
    let name = Arc::new(field.full_name().to_string());
    if rules.required() {
        push(&mut fns, name.clone(), Arc::new(move |val: &DateTime<Utc>, _: &TimestampRules, name: &String| {
            if val.second() == 0 && val.nanosecond() == 0 {
                return Err(format_err!("{}: is required", name));
            }
            Ok(true)
        }));
    }
    if rules.r#const.is_some() {
        push(&mut fns, name.clone(), Arc::new(move |val: &DateTime<Utc>, rules: &TimestampRules, name: &String| {
            let want = rules.r#const.unwrap().as_datetime();
            if *val != want {
                return Err(format_err!("{}: must be {}", name, want));
            }
            Ok(true)
        }));
    }
    if rules.lt.is_some() {
        push(&mut fns, name.clone(), Arc::new(move |val: &DateTime<Utc>, rules: &TimestampRules, name: &String| {
            let want = rules.lt.unwrap().as_datetime();
            if *val >= want {
                return Err(format_err!("{}: must be lt {}", name, want));
            }
            Ok(true)
        }));
    }
    if rules.lte.is_some() {
        push(&mut fns, name.clone(), Arc::new(move |val: &DateTime<Utc>, rules: &TimestampRules, name: &String| {
            let want = rules.lte.unwrap().as_datetime();
            if *val > want {
                return Err(format_err!("{}: must be lte {}", name, want));
            }
            Ok(true)
        }));
    }
    if rules.gt.is_some() {
        push(&mut fns, name.clone(), Arc::new(move |val: &DateTime<Utc>, rules: &TimestampRules, name: &String| {
            let want = rules.gt.unwrap().as_datetime();
            if *val <= want {
                return Err(format_err!("{}: must be gt {}", name, want));
            }
            Ok(true)
        }));
    }
    if rules.gte.is_some() {
        push(&mut fns, name.clone(), Arc::new(move |val: &DateTime<Utc>, rules: &TimestampRules, name: &String| {
            let want = rules.gte.unwrap().as_datetime();
            if *val < want {
                return Err(format_err!("{}: must be gte {}", name, want));
            }
            Ok(true)
        }));
    }
    if rules.lt_now() {
        push(&mut fns, name.clone(), Arc::new(move |val: &DateTime<Utc>, _: &TimestampRules, name: &String| {
            let now = Utc::now();
            if *val >= now {
                return Err(format_err!("{}: must be gte {}", name, now));
            }
            Ok(true)
        }));
    }
    if rules.gt_now() {
        push(&mut fns, name.clone(), Arc::new(move |val: &DateTime<Utc>, _: &TimestampRules, name: &String| {
            let now = Utc::now();
            if *val <= now {
                return Err(format_err!("{}: must be gte {}", name, now));
            }
            Ok(true)
        }));
    }
    if rules.within.is_some() {
        push(&mut fns, name.clone(), Arc::new(move |val: &DateTime<Utc>, rules: &TimestampRules, name: &String| {
            let now = Utc::now();
            let d = rules.within.unwrap().as_duration();
            if *val < now - d || *val > now + d {
                return Err(format_err!("{}: must be within {} from {}", name, d, now));
            }
            Ok(true)
        }));
    }
    fns
}
