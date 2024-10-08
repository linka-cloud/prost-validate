use crate::registry::NestedValidationFn;
use prost_reflect::{DynamicMessage, FieldDescriptor};
use prost_types::Timestamp;
use prost_validate::errors::{message, timestamp};
use prost_validate::utils::{AsDateTime, AsDuration};
use prost_validate::{errors, Error};
use prost_validate_types::field_rules::Type;
use prost_validate_types::{FieldRules, TimestampRules};
use std::sync::Arc;
use time::OffsetDateTime;

fn push<F>(fns: &mut Vec<NestedValidationFn<Box<DynamicMessage>>>, name: &Arc<String>, f: Arc<F>)
where
    F: Fn(&OffsetDateTime, &TimestampRules, &String) -> prost_validate::Result<bool>
        + Send
        + Sync
        + 'static,
{
    let name = name.clone();
    fns.push(Arc::new(move |val, rules, _| {
        let val = match val.map(|v| v.transcode_to::<Timestamp>()) {
            Some(Ok(val)) => val.as_datetime(),
            #[allow(clippy::unwrap_used)]
            _ => OffsetDateTime::from_unix_timestamp(0).unwrap(),
        };
        let rules = match rules.r#type {
            Some(Type::Timestamp(rules)) => rules,
            _ => {
                return Err(Error::new(
                    name.to_string(),
                    errors::Error::InvalidRules("unexpected timestamp rules".to_string()),
                ))
            }
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
                move |val: &OffsetDateTime, rules: &TimestampRules, name: &String| {
                    let want = rules.r#const.unwrap().as_datetime();
                    if *val != want {
                        return Err(Error::new(name.to_string(), timestamp::Error::Const(want)));
                    }
                    Ok(true)
                },
            ),
        );
    }

    // reference implementation: https://github.com/bufbuild/protoc-gen-validate/blob/v1.1.0/templates/goshared/timestamp.go
    if let Some(lt) = rules.lt.map(|v| v.as_datetime()) {
        if let Some(gt) = rules.gt.map(|v| v.as_datetime()) {
            if lt > gt {
                push(
                    &mut fns,
                    &name,
                    Arc::new(
                        move |val: &OffsetDateTime, rules: &TimestampRules, name: &String| {
                            let lt = rules.lt.unwrap().as_datetime();
                            let gt = rules.gt.unwrap().as_datetime();
                            if *val <= gt || *val >= lt {
                                return Err(Error::new(
                                    name,
                                    timestamp::Error::in_range(false, gt, lt, false),
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
                        move |val: &OffsetDateTime, rules: &TimestampRules, name: &String| {
                            let lt = rules.lt.unwrap().as_datetime();
                            let gt = rules.gt.unwrap().as_datetime();
                            if *val >= lt && *val <= gt {
                                return Err(Error::new(
                                    name,
                                    timestamp::Error::not_in_range(true, lt, gt, true),
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
                    Arc::new(
                        move |val: &OffsetDateTime, rules: &TimestampRules, name: &String| {
                            let gte = rules.gte.unwrap().as_datetime();
                            let lt = rules.lt.unwrap().as_datetime();
                            if *val < gte || *val >= lt {
                                return Err(Error::new(
                                    name,
                                    timestamp::Error::in_range(true, gte, lt, false),
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
                        move |val: &OffsetDateTime, rules: &TimestampRules, name: &String| {
                            let gte = rules.gte.unwrap().as_datetime();
                            let lt = rules.lt.unwrap().as_datetime();
                            if *val >= lt && *val < gte {
                                return Err(Error::new(
                                    name,
                                    timestamp::Error::not_in_range(true, lt, gte, false),
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
                    move |val: &OffsetDateTime, rules: &TimestampRules, name: &String| {
                        let lt = rules.lt.unwrap().as_datetime();
                        if *val >= lt {
                            return Err(Error::new(name.to_string(), timestamp::Error::Lt(lt)));
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
                    Arc::new(
                        move |val: &OffsetDateTime, rules: &TimestampRules, name: &String| {
                            let gt = rules.gt.unwrap().as_datetime();
                            let lte = rules.lte.unwrap().as_datetime();
                            if *val <= gt || *val > lte {
                                return Err(Error::new(
                                    name,
                                    timestamp::Error::in_range(false, gt, lte, true),
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
                        move |val: &OffsetDateTime, rules: &TimestampRules, name: &String| {
                            let gt = rules.gt.unwrap().as_datetime();
                            let lte = rules.lte.unwrap().as_datetime();
                            if *val > lte && *val <= gt {
                                return Err(Error::new(
                                    name,
                                    timestamp::Error::not_in_range(false, lte, gt, true),
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
                    Arc::new(
                        move |val: &OffsetDateTime, rules: &TimestampRules, name: &String| {
                            let gte = rules.gte.unwrap().as_datetime();
                            let lte = rules.lte.unwrap().as_datetime();
                            if *val < gte || *val > lte {
                                return Err(Error::new(
                                    name,
                                    timestamp::Error::in_range(true, gte, lte, true),
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
                        move |val: &OffsetDateTime, rules: &TimestampRules, name: &String| {
                            let gte = rules.gte.unwrap().as_datetime();
                            let lte = rules.lte.unwrap().as_datetime();
                            if *val > lte && *val < gte {
                                return Err(Error::new(
                                    name,
                                    timestamp::Error::not_in_range(false, lte, gte, false),
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
                    move |val: &OffsetDateTime, rules: &TimestampRules, name: &String| {
                        let lte = rules.lte.unwrap().as_datetime();
                        if *val > lte {
                            return Err(Error::new(name.to_string(), timestamp::Error::Lte(lte)));
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
                move |val: &OffsetDateTime, rules: &TimestampRules, name: &String| {
                    let gt = rules.gt.unwrap().as_datetime();
                    if *val <= gt {
                        return Err(Error::new(name.to_string(), timestamp::Error::Gt(gt)));
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
                move |val: &OffsetDateTime, rules: &TimestampRules, name: &String| {
                    let gte = rules.gte.unwrap().as_datetime();
                    if *val < gte {
                        return Err(Error::new(name.to_string(), timestamp::Error::Gte(gte)));
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
                Arc::new(
                    move |val: &OffsetDateTime, rules: &TimestampRules, name: &String| {
                        let now = time::OffsetDateTime::now_utc();
                        let d = rules.within.unwrap().as_duration();
                        if *val >= now || *val < now - d {
                            return Err(Error::new(name, timestamp::Error::LtNowWithin(d)));
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
                    move |val: &OffsetDateTime, _: &TimestampRules, name: &String| {
                        let now = OffsetDateTime::now_utc();
                        if *val >= now {
                            return Err(Error::new(name.to_string(), timestamp::Error::LtNow));
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
                Arc::new(
                    move |val: &OffsetDateTime, rules: &TimestampRules, name: &String| {
                        let now = OffsetDateTime::now_utc();
                        let d = rules.within.unwrap().as_duration();
                        if *val <= now || *val > now + d {
                            return Err(Error::new(name, timestamp::Error::GtNowWithin(d)));
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
                    move |val: &OffsetDateTime, _: &TimestampRules, name: &String| {
                        let now = OffsetDateTime::now_utc();
                        if *val <= now {
                            return Err(Error::new(name.to_string(), timestamp::Error::GtNow));
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
            Arc::new(
                move |val: &OffsetDateTime, rules: &TimestampRules, name: &String| {
                    let now = OffsetDateTime::now_utc();
                    let d = rules.within.unwrap().as_duration();
                    if *val < now - d || *val > now + d {
                        return Err(Error::new(name.to_string(), timestamp::Error::Within(d)));
                    }
                    Ok(true)
                },
            ),
        );
    }
    fns
}
