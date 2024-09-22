use crate::registry::NestedValidationFn;
use crate::utils::AsDuration;
use crate::validate_proto::field_rules::Type;
use crate::validate_proto::{DurationRules, FieldRules};
use anyhow::format_err;
use prost_reflect::{DynamicMessage, FieldDescriptor};
use prost_types::Duration;
use std::sync::Arc;
use time::Duration as TimeDelta;

fn push<F>(fns: &mut Vec<NestedValidationFn<Box<DynamicMessage>>>, name: &Arc<String>, f: Arc<F>)
where
    F: Fn(TimeDelta, &DurationRules, &String) -> anyhow::Result<bool> + Send + Sync + 'static,
{
    let name = name.clone();
    fns.push(Arc::new(move |val, rules, _| {
        let val = match val.map(|v| v.transcode_to::<Duration>()) {
            Some(Ok(val)) => val.as_duration(),
            _ => TimeDelta::default(),
        };
        let rules = match &rules.r#type {
            Some(Type::Duration(rules)) => rules,
            _ => return Err(format_err!("unexpected duration rules")),
        };
        f(val, rules, &name)
    }))
}

#[allow(clippy::unwrap_used)]
pub(crate) fn make_validate_duration(field: &FieldDescriptor, rules: &FieldRules) -> Vec<NestedValidationFn<Box<DynamicMessage>>> {
    let mut fns = Vec::new();
    let rules = match &rules.r#type {
        Some(Type::Duration(rules)) => rules,
        _ => return fns,
    };
    let name = Arc::new(field.full_name().to_string());
    if rules.required() {
        let name = name.clone();
        fns.push(Arc::new(move |val, _, _| {
            if val.is_none() {
                return Err(format_err!("{}: is required", name));
            }
            Ok(true)
        }));
    } else {
        fns.push(Arc::new(move |val, _, _| {
            Ok(val.is_some())
        }));
    }
    if rules.r#const.is_some() {
        push(&mut fns, &name, Arc::new(move |val: TimeDelta, rules: &DurationRules, name: &String| {
            let want = rules.r#const.unwrap().as_duration();
            if val != want {
                return Err(format_err!("{}: must be {}", name, want.to_string()));
            }
            Ok(true)
        }));
    }
    // reference implementation: https://github.com/bufbuild/protoc-gen-validate/blob/v1.1.0/templates/goshared/duration.go
    if let Some(lt) = rules.lt.map(|v| v.as_duration()) {
        if let Some(gt) = rules.gt.map(|v| v.as_duration()) {
            if lt > gt {
                push(&mut fns, &name, Arc::new(move |val: TimeDelta, rules: &DurationRules, name: &String| {
                    let lt = rules.lt.unwrap().as_duration();
                    let gt = rules.gt.unwrap().as_duration();
                    if val <= gt || val >= lt {
                        return Err(format_err!("{}: must be inside range ({}, {})", name, gt.to_string(), lt.to_string()));
                    }
                    Ok(true)
                }));
            } else {
                push(&mut fns, &name, Arc::new(move |val: TimeDelta, rules: &DurationRules, name: &String| {
                    let lt = rules.lt.unwrap().as_duration();
                    let gt = rules.gt.unwrap().as_duration();
                    if val >= lt && val <= gt {
                        return Err(format_err!("{}: must be outside range [{}, {}]", name, lt.to_string(), gt.to_string()));
                    }
                    Ok(true)
                }));
            }
        } else if let Some(gte) = rules.gte.map(|v| v.as_duration()) {
            if lt > gte {
                push(&mut fns, &name, Arc::new(move |val: TimeDelta, rules: &DurationRules, name: &String| {
                    let lt = rules.lt.unwrap().as_duration();
                    let gte = rules.gte.unwrap().as_duration();
                    if val < gte || val >= lt {
                        return Err(format_err!("{}: must be inside range [{}, {})", name, gte.to_string(), lt.to_string()));
                    }
                    Ok(true)
                }));
            } else {
                push(&mut fns, &name, Arc::new(move |val: TimeDelta, rules: &DurationRules, name: &String| {
                    let lt = rules.lt.unwrap().as_duration();
                    let gte = rules.gte.unwrap().as_duration();
                    if val >= lt && val < gte {
                        return Err(format_err!("{}: must be outside range [{}, {})", name, lt.to_string(), gte.to_string()));
                    }
                    Ok(true)
                }));
            }
        } else {
            push(&mut fns, &name, Arc::new(move |val: TimeDelta, rules: &DurationRules, name: &String| {
                let lt = rules.lt.unwrap().as_duration();
                if val >= lt {
                    return Err(format_err!("{}: must be less than {}", name, lt.to_string()));
                }
                Ok(true)
            }));
        }
    } else if let Some(lte) = rules.lte.map(|v| v.as_duration()) {
        if let Some(gt) = rules.gt.map(|v| v.as_duration()) {
            if lte > gt {
                push(&mut fns, &name, Arc::new(move |val: TimeDelta, rules: &DurationRules, name: &String| {
                    let lte = rules.lte.unwrap().as_duration();
                    let gt = rules.gt.unwrap().as_duration();
                    if val <= gt || val > lte {
                        return Err(format_err!("{}: must be inside range ({}, {}]", name, gt.to_string(), lte.to_string()));
                    }
                    Ok(true)
                }));
            } else {
                push(&mut fns, &name, Arc::new(move |val: TimeDelta, rules: &DurationRules, name: &String| {
                    let lte = rules.lte.unwrap().as_duration();
                    let gt = rules.gt.unwrap().as_duration();
                    if val >= lte && val < gt {
                        return Err(format_err!("{}: must be outside range ({}, {}]", name, lte.to_string(), gt.to_string()));
                    }
                    Ok(true)
                }));
            }
        } else if let Some(gte) = rules.gte.map(|v| v.as_duration()) {
            if lte > gte {
                push(&mut fns, &name, Arc::new(move |val: TimeDelta, rules: &DurationRules, name: &String| {
                    let lte = rules.lte.unwrap().as_duration();
                    let gte = rules.gte.unwrap().as_duration();
                    if val < gte || val > lte {
                        return Err(format_err!("{}: must be inside range [{}, {}]", name, gte.to_string(), lte.to_string()));
                    }
                    Ok(true)
                }));
            } else {
                push(&mut fns, &name, Arc::new(move |val: TimeDelta, rules: &DurationRules, name: &String| {
                    let lte = rules.lte.unwrap().as_duration();
                    let gte = rules.gte.unwrap().as_duration();
                    if val > lte && val < gte {
                        return Err(format_err!("{}: must be outside range ({}, {})", name, lte.to_string(), gte.to_string()));
                    }
                    Ok(true)
                }));
            }
        } else {
            push(&mut fns, &name, Arc::new(move |val: TimeDelta, rules: &DurationRules, name: &String| {
                let lte = rules.lte.unwrap().as_duration();
                if val > lte {
                    return Err(format_err!("{}: must be less than or equal to {}", name.to_string(), lte.to_string()));
                }
                Ok(true)
            }));
        }
    } else if rules.gt.is_some() {
        push(&mut fns, &name, Arc::new(move |val: TimeDelta, rules: &DurationRules, name: &String| {
            let gt = rules.gt.unwrap().as_duration();
            if val <= gt {
                return Err(format_err!("{}: must be greater than {}", name, gt.to_string()));
            }
            Ok(true)
        }));
    } else if rules.gte.is_some() {
        push(&mut fns, &name, Arc::new(move |val: TimeDelta, rules: &DurationRules, name: &String| {
            let gte = rules.gte.unwrap().as_duration();
            if val < gte {
                return Err(format_err!("{}: must be greater or equal to {}", name, gte.to_string()));
            }
            Ok(true)
        }));
    }

    if !rules.r#in.is_empty() {
        push(&mut fns, &name, Arc::new(move |val: TimeDelta, rules: &DurationRules, name: &String| {
            let vals = rules.r#in.iter().map(|v| v.as_duration()).collect::<Vec<TimeDelta>>();
            if !vals.contains(&val) {
                return Err(format_err!("{}: must be in {:?}", name, vals));
            }
            Ok(true)
        }));
    }
    if !rules.not_in.is_empty() {
        push(&mut fns, &name, Arc::new(move |val: TimeDelta, rules: &DurationRules, name: &String| {
            let vals = rules.not_in.iter().map(|v| v.as_duration()).collect::<Vec<TimeDelta>>();
            if vals.contains(&val) {
                return Err(format_err!("{}: must not be in {:?}", name, vals));
            }
            Ok(true)
        }));
    }
    fns
}
