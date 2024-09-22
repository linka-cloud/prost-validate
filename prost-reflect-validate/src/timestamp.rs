use crate::registry::NestedValidationFn;
use crate::utils::{AsDateTime, AsDuration};
use crate::validate_proto::field_rules::Type;
use crate::validate_proto::{FieldRules, TimestampRules};
use anyhow::format_err;
use prost_reflect::{DynamicMessage, FieldDescriptor};
use prost_types::Timestamp;
use std::sync::Arc;
use time::OffsetDateTime;

fn push<F>(fns: &mut Vec<NestedValidationFn<Box<DynamicMessage>>>, name: &Arc<String>, f: Box<F>)
where
    F: Fn(&OffsetDateTime, &TimestampRules, &String) -> anyhow::Result<bool>
        + Send
        + Sync
        + 'static,
{
    let name = name.clone();
    fns.push(Box::new(move |val, rules, _| {
        let val = match val.map(|v| v.transcode_to::<Timestamp>()) {
            Some(Ok(val)) => val.as_datetime(),
            #[allow(clippy::unwrap_used)]
            _ => OffsetDateTime::from_unix_timestamp(0).unwrap(),
        };
        let rules = match rules.r#type {
            Some(Type::Timestamp(rules)) => rules,
            _ => return Err(format_err!("unexpected timestamp rules")),
        };
        f(&val, &rules, &name)
    }))
}

#[allow(clippy::unwrap_used)]
pub(crate) fn make_validate_timestamp(
    field: &FieldDescriptor,
    rules: &FieldRules,
) -> Vec<NestedValidationFn<Box<DynamicMessage>>> {
    let mut fns = Vec::new();
    let rules = match rules.r#type {
        Some(Type::Timestamp(rules)) => rules,
        _ => return fns,
    };
    let name = Arc::new(field.full_name().to_string());
    if rules.required() {
        let name = name.clone();
        fns.push(Box::new(move |val, _, _| {
            if val.is_none() {
                return Err(format_err!("{}: is required", name));
            }
            Ok(true)
        }));
    } else {
        fns.push(Box::new(move |val, _, _| Ok(val.is_some())));
    }
    if rules.r#const.is_some() {
        push(
            &mut fns,
            &name,
            Box::new(
                move |val: &OffsetDateTime, rules: &TimestampRules, name: &String| {
                    let want = rules.r#const.unwrap().as_datetime();
                    if *val != want {
                        return Err(format_err!("{}: must be {}", name, want));
                    }
                    Ok(true)
                },
            ),
        );
    }

    if let Some(lt) = rules.lt.map(|v| v.as_datetime()) {
        if let Some(gt) = rules.gt.map(|v| v.as_datetime()) {
            if lt > gt {
                push(
                    &mut fns,
                    &name,
                    Box::new(
                        move |val: &OffsetDateTime, rules: &TimestampRules, name: &String| {
                            let lt = rules.lt.unwrap().as_datetime();
                            let gt = rules.gt.unwrap().as_datetime();
                            if *val <= gt || *val >= lt {
                                return Err(format_err!(
                                    "{}: must be inside range ({}, {})",
                                    name,
                                    gt,
                                    lt
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
                    Box::new(
                        move |val: &OffsetDateTime, rules: &TimestampRules, name: &String| {
                            let lt = rules.lt.unwrap().as_datetime();
                            let gt = rules.gt.unwrap().as_datetime();
                            if *val >= lt && *val <= gt {
                                return Err(format_err!(
                                    "{}: must be outside range [{}, {}]",
                                    name,
                                    gt,
                                    lt
                                ));
                            }
                            Ok(true)
                        },
                    ),
                );
            }
        } else if let Some(gte) = rules.gte.map(|v| v.as_datetime()) {
            if lt > gte {
                push(
                    &mut fns,
                    &name,
                    Box::new(
                        move |val: &OffsetDateTime, rules: &TimestampRules, name: &String| {
                            let gte = rules.gte.unwrap().as_datetime();
                            let lt = rules.lt.unwrap().as_datetime();
                            if *val < gte || *val >= lt {
                                return Err(format_err!(
                                    "{}: must be inside range [{}, {})",
                                    name,
                                    gte,
                                    lt
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
                    Box::new(
                        move |val: &OffsetDateTime, rules: &TimestampRules, name: &String| {
                            let gte = rules.gte.unwrap().as_datetime();
                            let lt = rules.lt.unwrap().as_datetime();
                            if *val >= lt && *val < gte {
                                return Err(format_err!(
                                    "{}: must be outside range [{}, {})",
                                    name,
                                    lt,
                                    gte
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
                Box::new(
                    move |val: &OffsetDateTime, rules: &TimestampRules, name: &String| {
                        let lt = rules.lt.unwrap().as_datetime();
                        if *val >= lt {
                            return Err(format_err!("{}: must be less than {}", name, lt));
                        }
                        Ok(true)
                    },
                ),
            );
        }
    } else if let Some(lte) = rules.lte.map(|v| v.as_datetime()) {
        if let Some(gt) = rules.gt.map(|v| v.as_datetime()) {
            if lte > gt {
                push(
                    &mut fns,
                    &name,
                    Box::new(
                        move |val: &OffsetDateTime, rules: &TimestampRules, name: &String| {
                            let gt = rules.gt.unwrap().as_datetime();
                            let lte = rules.lte.unwrap().as_datetime();
                            if *val <= gt || *val > lte {
                                return Err(format_err!(
                                    "{}: must be inside range ({}, {}]",
                                    name,
                                    gt,
                                    lte
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
                    Box::new(
                        move |val: &OffsetDateTime, rules: &TimestampRules, name: &String| {
                            let gt = rules.gt.unwrap().as_datetime();
                            let lte = rules.lte.unwrap().as_datetime();
                            if *val > lte && *val <= gt {
                                return Err(format_err!(
                                    "{}: must be outside range ({}, {}]",
                                    name,
                                    lte,
                                    gt
                                ));
                            }
                            Ok(true)
                        },
                    ),
                );
            }
        } else if let Some(gte) = rules.gte.map(|v| v.as_datetime()) {
            if lte > gte {
                push(
                    &mut fns,
                    &name,
                    Box::new(
                        move |val: &OffsetDateTime, rules: &TimestampRules, name: &String| {
                            let gte = rules.gte.unwrap().as_datetime();
                            let lte = rules.lte.unwrap().as_datetime();
                            if *val < gte || *val > lte {
                                return Err(format_err!(
                                    "{}: must be inside range [{}, {}]",
                                    name,
                                    gte,
                                    lte
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
                    Box::new(
                        move |val: &OffsetDateTime, rules: &TimestampRules, name: &String| {
                            let gte = rules.gte.unwrap().as_datetime();
                            let lte = rules.lte.unwrap().as_datetime();
                            if *val > lte && *val < gte {
                                return Err(format_err!(
                                    "{}: must be outside range ({}, {}]",
                                    name,
                                    lte,
                                    gte
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
                Box::new(
                    move |val: &OffsetDateTime, rules: &TimestampRules, name: &String| {
                        let lte = rules.lte.unwrap().as_datetime();
                        if *val > lte {
                            return Err(format_err!(
                                "{}: must be less than or equal to {}",
                                name,
                                lte
                            ));
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
            Box::new(
                move |val: &OffsetDateTime, rules: &TimestampRules, name: &String| {
                    let gt = rules.gt.unwrap().as_datetime();
                    if *val <= gt {
                        return Err(format_err!("{}: must be greater than {}", name, gt));
                    }
                    Ok(true)
                },
            ),
        );
    } else if rules.gte.is_some() {
        push(
            &mut fns,
            &name,
            Box::new(
                move |val: &OffsetDateTime, rules: &TimestampRules, name: &String| {
                    let gte = rules.gte.unwrap().as_datetime();
                    if *val < gte {
                        return Err(format_err!(
                            "{}: must be greater than or equal to {}",
                            name,
                            gte
                        ));
                    }
                    Ok(true)
                },
            ),
        );
    } else if rules.lt_now.is_some() {
        if rules.within.is_some() {
            push(
                &mut fns,
                &name,
                Box::new(
                    move |val: &OffsetDateTime, rules: &TimestampRules, name: &String| {
                        // let now = Utc::now();
                        let now = time::OffsetDateTime::now_utc();
                        let d = rules.within.unwrap().as_duration();
                        if *val >= now || *val < now - d {
                            return Err(format_err!(
                                "{}: must be within {} from {}",
                                name,
                                d.to_string(),
                                now
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
                Box::new(
                    move |val: &OffsetDateTime, _: &TimestampRules, name: &String| {
                        let now = OffsetDateTime::now_utc();
                        if *val >= now {
                            return Err(format_err!("{}: must be lt {}", name, now));
                        }
                        Ok(true)
                    },
                ),
            );
        }
    } else if rules.gt_now.is_some() {
        if rules.within.is_some() {
            push(
                &mut fns,
                &name,
                Box::new(
                    move |val: &OffsetDateTime, rules: &TimestampRules, name: &String| {
                        let now = OffsetDateTime::now_utc();
                        let d = rules.within.unwrap().as_duration();
                        if *val <= now || *val > now + d {
                            return Err(format_err!(
                                "{}: value must be less than now within {}",
                                name,
                                d.to_string()
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
                Box::new(
                    move |val: &OffsetDateTime, _: &TimestampRules, name: &String| {
                        let now = OffsetDateTime::now_utc();
                        if *val <= now {
                            return Err(format_err!("{}: must be gt {}", name, now));
                        }
                        Ok(true)
                    },
                ),
            );
        }
    } else if rules.within.is_some() {
        push(
            &mut fns,
            &name,
            Box::new(
                move |val: &OffsetDateTime, rules: &TimestampRules, name: &String| {
                    let now = OffsetDateTime::now_utc();
                    let d = rules.within.unwrap().as_duration();
                    if *val < now - d || *val > now + d {
                        return Err(format_err!(
                            "{}: must be within {} from {}",
                            name,
                            d.to_string(),
                            now
                        ));
                    }
                    Ok(true)
                },
            ),
        );
    }
    fns
}
