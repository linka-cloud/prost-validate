use crate::registry::FieldValidationFn;
use crate::utils::AsDuration;
use crate::validate_proto::field_rules::Type;
use crate::validate_proto::{DurationRules, FieldRules};
use anyhow::{format_err};
use chrono::TimeDelta;
use prost_reflect::{DynamicMessage, FieldDescriptor};
use prost_types::Duration;
use std::sync::Arc;

fn push<F>(fns: &mut Vec<FieldValidationFn<Box<DynamicMessage>>>, name: Arc<String>, f: Arc<F>)
where
    F: Fn(TimeDelta, &DurationRules, &String) -> anyhow::Result<bool> + Send + Sync + 'static,
{
    let name = name.clone();
    fns.push(Arc::new(move |val, rules| {
        let val = match val {
            Some(val) => val.transcode_to::<Duration>().unwrap().as_duration(),
            _ => TimeDelta::new(0, 0).unwrap(),
        };
        let rules = match &rules.r#type {
            Some(Type::Duration(rules)) => rules,
            _ => return Err(format_err!("unexpected duration rules")),
        };
        f(val, &rules, &name)
    }))
}

pub(crate) fn make_validate_duration(field: &FieldDescriptor, rules: &FieldRules) -> Vec<FieldValidationFn<Box<DynamicMessage>>> {
    let mut fns = Vec::new();
    let rules = match &rules.r#type {
        Some(Type::Duration(rules)) => rules,
        _ => return fns,
    };
    let name = Arc::new(field.full_name().to_string());
    if rules.required() {
        push(&mut fns, name.clone(), Arc::new(move |val: TimeDelta, _: &DurationRules, name: &String| {
            if val.is_zero() {
                return Err(format_err!("{}: is required", name));
            }
            Ok(true)
        }));
    }
    if rules.r#const.is_some() {
        push(&mut fns, name.clone(), Arc::new(move |val: TimeDelta, rules: &DurationRules, name: &String| {
            let want = rules.r#const.unwrap().as_duration();
            if val != want {
                return Err(format_err!("{}: must be {}", name, want));
            }
            Ok(true)
        }));
    }
    if rules.lt.is_some() {
        push(&mut fns, name.clone(), Arc::new(move |val: TimeDelta, rules: &DurationRules, name: &String| {
            let want = rules.lt.as_duration();
            if val >= want {
                return Err(format_err!("{}: must be lt {}", name, want));
            }
            Ok(true)
        }));
    }
    if rules.lte.is_some() {
        push(&mut fns, name.clone(), Arc::new(move |val: TimeDelta, rules: &DurationRules, name: &String| {
            let want = rules.lte.as_duration();
            if val > want {
                return Err(format_err!("{}: must be lte {}", name, want));
            }
            Ok(true)
        }));
    }
    if rules.gt.is_some() {
        push(&mut fns, name.clone(), Arc::new(move |val: TimeDelta, rules: &DurationRules, name: &String| {
            let want = rules.gt.as_duration();
            if val <= want {
                return Err(format_err!("{}: must be gt {}", name, want));
            }
            Ok(true)
        }));
    }
    if rules.gte.is_some() {
        push(&mut fns, name.clone(), Arc::new(move |val: TimeDelta, rules: &DurationRules, name: &String| {
            let want = rules.gte.as_duration();
            if val < want {
                return Err(format_err!("{}: must be gte {}", name, want));
            }
            Ok(true)
        }));
    }
    if !rules.r#in.is_empty() {
        push(&mut fns, name.clone(), Arc::new(move |val: TimeDelta, rules: &DurationRules, name: &String| {
            let vals = rules.r#in.iter().map(|v| v.as_duration()).collect::<Vec<TimeDelta>>();
            if !vals.contains(&val) {
                return Err(format_err!("{}: must be in {:?}", name, vals));
            }
            Ok(true)
        }));
    }
    if !rules.not_in.is_empty() {
        push(&mut fns, name.clone(), Arc::new(move |val: TimeDelta, rules: &DurationRules, name: &String| {
            let vals = rules.not_in.iter().map(|v| v.as_duration()).collect::<Vec<TimeDelta>>();
            if !vals.contains(&val) {
                return Err(format_err!("{}: must not be in {:?}", name, vals));
            }
            Ok(true)
        }));
    }
    fns
}
