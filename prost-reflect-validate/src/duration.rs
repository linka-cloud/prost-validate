use crate::registry::NestedValidationFn;
use prost_reflect::{DynamicMessage, FieldDescriptor};
use prost_types::Duration;
use prost_validate::errors::{duration, message};
use prost_validate::utils::AsDuration;
use prost_validate::{errors, Error};
use prost_validate_types::field_rules::Type;
use prost_validate_types::{DurationRules, FieldRules};
use std::sync::Arc;
use time::Duration as TimeDelta;

fn push<F>(fns: &mut Vec<NestedValidationFn<Box<DynamicMessage>>>, name: &Arc<String>, f: Arc<F>)
where
    F: Fn(TimeDelta, &DurationRules, &String) -> prost_validate::Result<bool>
        + Send
        + Sync
        + 'static,
{
    let name = name.clone();
    fns.push(Arc::new(move |val, rules, _| {
        let val = match val.map(|v| v.transcode_to::<Duration>()) {
            Some(Ok(val)) => val.as_duration(),
            _ => TimeDelta::default(),
        };
        let rules = match &rules.r#type {
            Some(Type::Duration(rules)) => rules,
            _ => {
                return Err(Error::new(
                    name.clone(),
                    errors::Error::InvalidRules("unexpected duration rules".to_string()),
                ))
            }
        };
        f(val, rules, &name)
    }))
}

#[allow(clippy::unwrap_used)]
pub(crate) fn make_validate_duration(
    field: &FieldDescriptor,
    rules: &FieldRules,
) -> Vec<NestedValidationFn<Box<DynamicMessage>>> {
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
                return Err(Error::new(name.to_string(), message::Error::Required));
            }
            Ok(true)
        }));
    } else {
        fns.push(Arc::new(move |val, _, _| Ok(val.is_some())));
    }
    if rules.r#const.is_some() {
        push(
            &mut fns,
            &name,
            Arc::new(
                move |val: TimeDelta, rules: &DurationRules, name: &String| {
                    let want = rules.r#const.unwrap().as_duration();
                    if val != want {
                        return Err(Error::new(name.to_string(), duration::Error::Const(want)));
                    }
                    Ok(true)
                },
            ),
        );
    }
    // reference implementation: https://github.com/bufbuild/protoc-gen-validate/blob/v1.1.0/templates/goshared/duration.go
    if let Some(lt) = rules.lt.map(|v| v.as_duration()) {
        if let Some(gt) = rules.gt.map(|v| v.as_duration()) {
            if lt > gt {
                push(
                    &mut fns,
                    &name,
                    Arc::new(
                        move |val: TimeDelta, rules: &DurationRules, name: &String| {
                            let lt = rules.lt.unwrap().as_duration();
                            let gt = rules.gt.unwrap().as_duration();
                            if val <= gt || val >= lt {
                                return Err(Error::new(
                                    name.to_string(),
                                    duration::Error::in_range(false, gt, lt, false),
                                ));
                            }
                            Ok(true)
                        },
                    ),
                );
            } else {
                push(
                    &mut fns,
                    &name,
                    Arc::new(
                        move |val: TimeDelta, rules: &DurationRules, name: &String| {
                            let lt = rules.lt.unwrap().as_duration();
                            let gt = rules.gt.unwrap().as_duration();
                            if val >= lt && val <= gt {
                                return Err(Error::new(
                                    name.to_string(),
                                    duration::Error::not_in_range(true, lt, gt, true),
                                ));
                            }
                            Ok(true)
                        },
                    ),
                );
            }
        } else if let Some(gte) = rules.gte.map(|v| v.as_duration()) {
            if lt > gte {
                push(
                    &mut fns,
                    &name,
                    Arc::new(
                        move |val: TimeDelta, rules: &DurationRules, name: &String| {
                            let lt = rules.lt.unwrap().as_duration();
                            let gte = rules.gte.unwrap().as_duration();
                            if val < gte || val >= lt {
                                return Err(Error::new(
                                    name.to_string(),
                                    duration::Error::in_range(true, gte, lt, false),
                                ));
                            }
                            Ok(true)
                        },
                    ),
                );
            } else {
                push(
                    &mut fns,
                    &name,
                    Arc::new(
                        move |val: TimeDelta, rules: &DurationRules, name: &String| {
                            let lt = rules.lt.unwrap().as_duration();
                            let gte = rules.gte.unwrap().as_duration();
                            if val >= lt && val < gte {
                                return Err(Error::new(
                                    name.to_string(),
                                    duration::Error::not_in_range(true, lt, gte, false),
                                ));
                            }
                            Ok(true)
                        },
                    ),
                );
            }
        } else {
            push(
                &mut fns,
                &name,
                Arc::new(
                    move |val: TimeDelta, rules: &DurationRules, name: &String| {
                        let lt = rules.lt.unwrap().as_duration();
                        if val >= lt {
                            return Err(Error::new(name.to_string(), duration::Error::Lt(lt)));
                        }
                        Ok(true)
                    },
                ),
            );
        }
    } else if let Some(lte) = rules.lte.map(|v| v.as_duration()) {
        if let Some(gt) = rules.gt.map(|v| v.as_duration()) {
            if lte > gt {
                push(
                    &mut fns,
                    &name,
                    Arc::new(
                        move |val: TimeDelta, rules: &DurationRules, name: &String| {
                            let lte = rules.lte.unwrap().as_duration();
                            let gt = rules.gt.unwrap().as_duration();
                            if val <= gt || val > lte {
                                return Err(Error::new(
                                    name.to_string(),
                                    duration::Error::in_range(false, gt, lte, true),
                                ));
                            }
                            Ok(true)
                        },
                    ),
                );
            } else {
                push(
                    &mut fns,
                    &name,
                    Arc::new(
                        move |val: TimeDelta, rules: &DurationRules, name: &String| {
                            let lte = rules.lte.unwrap().as_duration();
                            let gt = rules.gt.unwrap().as_duration();
                            if val >= lte && val < gt {
                                return Err(Error::new(
                                    name.to_string(),
                                    duration::Error::not_in_range(false, lte, gt, true),
                                ));
                            }
                            Ok(true)
                        },
                    ),
                );
            }
        } else if let Some(gte) = rules.gte.map(|v| v.as_duration()) {
            if lte > gte {
                push(
                    &mut fns,
                    &name,
                    Arc::new(
                        move |val: TimeDelta, rules: &DurationRules, name: &String| {
                            let lte = rules.lte.unwrap().as_duration();
                            let gte = rules.gte.unwrap().as_duration();
                            if val < gte || val > lte {
                                return Err(Error::new(
                                    name.to_string(),
                                    duration::Error::in_range(true, gte, lte, true),
                                ));
                            }
                            Ok(true)
                        },
                    ),
                );
            } else {
                push(
                    &mut fns,
                    &name,
                    Arc::new(
                        move |val: TimeDelta, rules: &DurationRules, name: &String| {
                            let lte = rules.lte.unwrap().as_duration();
                            let gte = rules.gte.unwrap().as_duration();
                            if val > lte && val < gte {
                                return Err(Error::new(
                                    name.to_string(),
                                    duration::Error::not_in_range(false, lte, gte, false),
                                ));
                            }
                            Ok(true)
                        },
                    ),
                );
            }
        } else {
            push(
                &mut fns,
                &name,
                Arc::new(
                    move |val: TimeDelta, rules: &DurationRules, name: &String| {
                        let lte = rules.lte.unwrap().as_duration();
                        if val > lte {
                            return Err(Error::new(name.to_string(), duration::Error::Lte(lte)));
                        }
                        Ok(true)
                    },
                ),
            );
        }
    } else if rules.gt.is_some() {
        push(
            &mut fns,
            &name,
            Arc::new(
                move |val: TimeDelta, rules: &DurationRules, name: &String| {
                    let gt = rules.gt.unwrap().as_duration();
                    if val <= gt {
                        return Err(Error::new(name.to_string(), duration::Error::Gt(gt)));
                    }
                    Ok(true)
                },
            ),
        );
    } else if rules.gte.is_some() {
        push(
            &mut fns,
            &name,
            Arc::new(
                move |val: TimeDelta, rules: &DurationRules, name: &String| {
                    let gte = rules.gte.unwrap().as_duration();
                    if val < gte {
                        return Err(Error::new(name.to_string(), duration::Error::Gte(gte)));
                    }
                    Ok(true)
                },
            ),
        );
    }

    if !rules.r#in.is_empty() {
        push(
            &mut fns,
            &name,
            Arc::new(
                move |val: TimeDelta, rules: &DurationRules, name: &String| {
                    let vals = rules
                        .r#in
                        .iter()
                        .map(|v| v.as_duration())
                        .collect::<Vec<TimeDelta>>();
                    if !vals.contains(&val) {
                        return Err(Error::new(name.to_string(), duration::Error::In(vals)));
                    }
                    Ok(true)
                },
            ),
        );
    }
    if !rules.not_in.is_empty() {
        push(
            &mut fns,
            &name,
            Arc::new(
                move |val: TimeDelta, rules: &DurationRules, name: &String| {
                    let vals = rules
                        .not_in
                        .iter()
                        .map(|v| v.as_duration())
                        .collect::<Vec<TimeDelta>>();
                    if vals.contains(&val) {
                        return Err(Error::new(name.to_string(), duration::Error::NotIn(vals)));
                    }
                    Ok(true)
                },
            ),
        );
    }
    fns
}
