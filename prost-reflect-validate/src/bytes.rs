use crate::registry::FieldValidationFn;
use prost::bytes::Bytes;
use prost_reflect::FieldDescriptor;
use prost_validate::errors::bytes;
use prost_validate::{format_err, Error};
use prost_validate_types::bytes_rules::WellKnown;
use prost_validate_types::field_rules::Type;
use prost_validate_types::{BytesRules, FieldRules};
use regex::bytes::Regex;
use std::ops::Deref;
use std::sync::Arc;

fn push<F>(fns: &mut Vec<FieldValidationFn<Arc<Bytes>>>, name: &Arc<String>, f: Arc<F>)
where
    F: Fn(&Bytes, &BytesRules, &String) -> prost_validate::Result<bool> + Send + Sync + 'static,
{
    let name = name.clone();
    fns.push(Arc::new(move |val, rules| {
        let default = Arc::new(Bytes::default());
        let val = val.unwrap_or(default);
        let rules = match &rules.r#type {
            Some(Type::Bytes(rules)) => rules,
            _ => return Err(format_err!(name, "unexpected string rules")),
        };
        f(&val, rules, &name)
    }))
}

pub(crate) fn make_validate_bytes(
    field: &FieldDescriptor,
    rules: &FieldRules,
) -> Vec<FieldValidationFn<Arc<Bytes>>> {
    let mut fns = Vec::new();
    if !matches!(rules.r#type, Some(Type::Bytes(_))) {
        return fns;
    }
    let rules = match &rules.r#type {
        Some(Type::Bytes(rule)) => rule,
        _ => return fns,
    };
    let name = Arc::new(field.full_name().to_string());
    if rules.ignore_empty() {
        push(
            &mut fns,
            &name,
            Arc::new(move |val: &Bytes, _: &BytesRules, _: &String| Ok(!val.is_empty())),
        );
    }
    if rules.r#const.is_some() {
        push(
            &mut fns,
            &name,
            Arc::new(move |val: &Bytes, rules: &BytesRules, name: &String| {
                if val != rules.r#const() {
                    return Err(Error::new(
                        name.to_string(),
                        bytes::Error::Const(rules.r#const().to_vec()),
                    ));
                }
                Ok(true)
            }),
        );
    }
    if rules.len.is_some() {
        push(
            &mut fns,
            &name,
            Arc::new(move |val: &Bytes, rules: &BytesRules, name: &String| {
                if val.len() != rules.len() as usize {
                    return Err(Error::new(
                        name.to_string(),
                        bytes::Error::Len(rules.len() as usize),
                    ));
                }
                Ok(true)
            }),
        );
    }
    if rules.min_len.is_some() {
        push(
            &mut fns,
            &name,
            Arc::new(move |val: &Bytes, rules: &BytesRules, name: &String| {
                if val.len() < rules.min_len() as usize {
                    return Err(Error::new(
                        name.to_string(),
                        bytes::Error::MinLen(rules.min_len() as usize),
                    ));
                }
                Ok(true)
            }),
        );
    }
    if rules.max_len.is_some() {
        push(
            &mut fns,
            &name,
            Arc::new(move |val: &Bytes, rules: &BytesRules, name: &String| {
                if val.len() > rules.max_len() as usize {
                    return Err(Error::new(
                        name.to_string(),
                        bytes::Error::MaxLen(rules.max_len() as usize),
                    ));
                }
                Ok(true)
            }),
        );
    }
    if rules.pattern.is_some() {
        push(
            &mut fns,
            &name,
            Arc::new(move |val: &Bytes, rules: &BytesRules, name: &String| {
                let regex = Regex::new(rules.pattern())
                    .map_err(|_| format_err!(name, "must be a valid regex pattern"))?;
                if !regex.is_match(val.iter().as_slice()) {
                    return Err(Error::new(
                        name.to_string(),
                        bytes::Error::Pattern(rules.pattern().to_string()),
                    ));
                }
                Ok(true)
            }),
        );
    }
    if rules.prefix.is_some() {
        push(
            &mut fns,
            &name,
            Arc::new(move |val: &Bytes, rules: &BytesRules, name: &String| {
                if !val.starts_with(rules.prefix()) {
                    return Err(Error::new(
                        name.to_string(),
                        bytes::Error::Prefix(rules.prefix().to_vec()),
                    ));
                }
                Ok(true)
            }),
        );
    }
    if rules.suffix.is_some() {
        push(
            &mut fns,
            &name,
            Arc::new(move |val: &Bytes, rules: &BytesRules, name: &String| {
                if !val.ends_with(rules.suffix()) {
                    return Err(Error::new(
                        name.to_string(),
                        bytes::Error::Suffix(rules.suffix().to_vec()),
                    ));
                }
                Ok(true)
            }),
        );
    }
    if rules.contains.is_some() {
        push(
            &mut fns,
            &name,
            Arc::new(move |val: &Bytes, rules: &BytesRules, name: &String| {
                let v = Bytes::from(rules.contains().to_vec());
                if !contains_slice(val, &v) {
                    return Err(Error::new(
                        name.to_string(),
                        bytes::Error::Contains(rules.contains().to_vec()),
                    ));
                }
                Ok(true)
            }),
        );
    }
    if !rules.r#in.is_empty() {
        push(
            &mut fns,
            &name,
            Arc::new(move |val: &Bytes, rules: &BytesRules, name: &String| {
                if !rules.r#in.contains(&val.deref().into()) {
                    return Err(Error::new(
                        name.to_string(),
                        bytes::Error::In(rules.r#in.clone()),
                    ));
                }
                Ok(true)
            }),
        );
    }
    if !rules.not_in.is_empty() {
        push(
            &mut fns,
            &name,
            Arc::new(move |val: &Bytes, rules: &BytesRules, name: &String| {
                if rules.not_in.contains(&val.deref().into()) {
                    return Err(Error::new(
                        name.to_string(),
                        bytes::Error::NotIn(rules.not_in.clone()),
                    ));
                }
                Ok(true)
            }),
        );
    }
    if rules.well_known.is_none() {
        return fns;
    }
    #[allow(clippy::unwrap_used)]
    match rules.well_known.unwrap() {
        WellKnown::Ip(v) => {
            if v {
                push(
                    &mut fns,
                    &name,
                    Arc::new(move |val: &Bytes, _: &BytesRules, name: &String| {
                        if val.len() != 16 && val.len() != 4 {
                            return Err(Error::new(name.to_string(), bytes::Error::Ip));
                        }
                        Ok(true)
                    }),
                );
            }
        }
        WellKnown::Ipv4(v) => {
            if v {
                push(
                    &mut fns,
                    &name,
                    Arc::new(move |val: &Bytes, _: &BytesRules, name: &String| {
                        if val.len() != 4 {
                            return Err(Error::new(name.to_string(), bytes::Error::Ipv4));
                        }
                        Ok(true)
                    }),
                );
            }
        }
        WellKnown::Ipv6(v) => {
            if v {
                push(
                    &mut fns,
                    &name,
                    Arc::new(move |val: &Bytes, _: &BytesRules, name: &String| {
                        if val.len() != 16 {
                            return Err(Error::new(name.to_string(), bytes::Error::Ipv6));
                        }
                        Ok(true)
                    }),
                );
            }
        }
    }
    fns
}

fn contains_slice(slice: &'_ Bytes, sub: &'_ Bytes) -> bool {
    let len = sub.len();
    if len == 0 {
        return true;
    }
    slice.windows(len).any(move |sub_slice| sub_slice == sub)
}
