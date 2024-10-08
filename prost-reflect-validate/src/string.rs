use crate::registry::FieldValidationFn;
use prost_reflect::FieldDescriptor;
use prost_validate::errors::string;
use prost_validate::ValidateStringExt;
use prost_validate::{format_err, Error};
use prost_validate_types::field_rules::Type;
use prost_validate_types::string_rules::WellKnown;
use prost_validate_types::{FieldRules, KnownRegex, StringRules};
use regex::Regex;
use std::ops::Deref;
use std::sync::Arc;

macro_rules! string_rules {
    ($rules:ident) => {
        match &$rules.r#type {
            Some(Type::String(rules)) => rules,
            _ => return Err(format_err!("unexpected string rules")),
        }
    };
}

fn push<F>(fns: &mut Vec<FieldValidationFn<String>>, name: &Arc<String>, f: Arc<F>)
where
    F: Fn(String, &StringRules, &String) -> prost_validate::Result<bool> + Send + Sync + 'static,
{
    let name = name.clone();
    fns.push(Arc::new(move |val, rules| {
        let val = val.unwrap_or("".to_string());
        let rules = string_rules!(rules);
        f(val, rules, &name)
    }))
}

pub(crate) fn make_validate_string(
    field: &FieldDescriptor,
    rules: &FieldRules,
) -> Vec<FieldValidationFn<String>> {
    let mut fns: Vec<FieldValidationFn<String>> = Vec::new();
    if !matches!(rules.r#type, Some(Type::String(_))) {
        return fns;
    }
    let rules = match &rules.r#type {
        Some(Type::String(rule)) => rule,
        _ => return fns,
    };
    if rules.ignore_empty() {
        fns.push(Arc::new(|val, _| {
            Ok(val.map(|v| !v.is_empty()).unwrap_or(false))
        }));
    }
    let name = Arc::new(field.full_name().to_string());
    if rules.r#const.is_some() {
        push(
            &mut fns,
            &name,
            Arc::new(move |val: String, rules: &StringRules, name: &String| {
                let v = rules.r#const();
                if val != v {
                    return Err(Error::new(
                        name.to_string(),
                        string::Error::Const(v.to_string()),
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
            Arc::new(move |val: String, rules: &StringRules, name: &String| {
                let v = rules.len() as usize;
                if val.chars().count() != v {
                    return Err(Error::new(name.to_string(), string::Error::Len(v)));
                }
                Ok(true)
            }),
        )
    }
    if rules.min_len.is_some() {
        push(
            &mut fns,
            &name,
            Arc::new(move |val: String, rules: &StringRules, name: &String| {
                let v = rules.min_len() as usize;
                if val.chars().count() < v {
                    return Err(Error::new(name.to_string(), string::Error::MinLen(v)));
                }
                Ok(true)
            }),
        );
    }
    if rules.max_len.is_some() {
        push(
            &mut fns,
            &name,
            Arc::new(move |val: String, rules: &StringRules, name: &String| {
                let v = rules.max_len() as usize;
                if val.chars().count() > v {
                    return Err(Error::new(name.to_string(), string::Error::MaxLen(v)));
                }
                Ok(true)
            }),
        );
    }
    if rules.len_bytes.is_some() {
        push(
            &mut fns,
            &name,
            Arc::new(move |val: String, rules: &StringRules, name: &String| {
                let v = rules.len_bytes() as usize;
                if val.len() != v {
                    return Err(Error::new(name.to_string(), string::Error::LenBytes(v)));
                }
                Ok(true)
            }),
        )
    }
    if rules.min_bytes.is_some() {
        push(
            &mut fns,
            &name,
            Arc::new(move |val: String, rules: &StringRules, name: &String| {
                let v = rules.min_bytes() as usize;
                if val.len() < v {
                    return Err(Error::new(name.to_string(), string::Error::MinLenBytes(v)));
                }
                Ok(true)
            }),
        );
    }
    if rules.max_bytes.is_some() {
        push(
            &mut fns,
            &name,
            Arc::new(move |val: String, rules: &StringRules, name: &String| {
                let v = rules.max_bytes() as usize;
                if val.len() > v {
                    return Err(Error::new(name.to_string(), string::Error::MaxLenBytes(v)));
                }
                Ok(true)
            }),
        );
    }
    if let Some(v) = &rules.pattern {
        let regex = Regex::new(v.as_str());
        push(
            &mut fns,
            &name,
            Arc::new(move |val: String, rules: &StringRules, name: &String| {
                let v = rules.pattern();
                let regex = match &regex {
                    Ok(r) => r,
                    Err(err) => {
                        return Err(Error::new(
                            name.to_string(),
                            string::Error::Pattern(err.to_string()),
                        ))
                    }
                };
                if !regex.is_match(val.as_str()) {
                    return Err(Error::new(
                        name.to_string(),
                        string::Error::Pattern(v.to_string()),
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
            Arc::new(move |val: String, rules: &StringRules, name: &String| {
                let v = rules.prefix();
                if !val.as_str().starts_with(v) {
                    return Err(Error::new(
                        name.to_string(),
                        string::Error::Prefix(v.to_string()),
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
            Arc::new(move |val: String, rules: &StringRules, name: &String| {
                let v = rules.suffix();
                if !val.as_str().ends_with(v) {
                    return Err(Error::new(
                        name.to_string(),
                        string::Error::Suffix(v.to_string()),
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
            Arc::new(move |val: String, rules: &StringRules, name: &String| {
                let v = rules.contains();
                if !val.contains(v) {
                    return Err(Error::new(
                        name.to_string(),
                        string::Error::Contains(v.to_string()),
                    ));
                }
                Ok(true)
            }),
        );
    }
    if rules.not_contains.is_some() {
        push(
            &mut fns,
            &name,
            Arc::new(move |val: String, rules: &StringRules, name: &String| {
                let v = rules.not_contains();
                if val.contains(v) {
                    return Err(Error::new(
                        name.to_string(),
                        string::Error::NotContains(v.to_string()),
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
            Arc::new(move |val: String, rules: &StringRules, name: &String| {
                let v = rules.r#in.deref();
                if !v.contains(&val.to_string()) {
                    return Err(Error::new(name.to_string(), string::Error::In(v.to_vec())));
                }
                Ok(true)
            }),
        );
    }
    if !rules.not_in.is_empty() {
        push(
            &mut fns,
            &name,
            Arc::new(move |val: String, rules: &StringRules, name: &String| {
                let v = rules.not_in.deref();
                if v.contains(&val.to_string()) {
                    return Err(Error::new(
                        name.to_string(),
                        string::Error::NotIn(v.to_vec()),
                    ));
                }
                Ok(true)
            }),
        );
    }
    if rules.well_known.is_none() {
        return fns;
    }
    let strict = rules.strict();
    #[allow(clippy::unwrap_used)]
    match rules.well_known.unwrap() {
        WellKnown::Email(v) => {
            if v {
                fns.push(Arc::new(move |val, _| {
                    let val = val.unwrap_or("".to_string());
                    if let Err(_) = val.validate_email() {
                        return Err(Error::new(name.to_string(), string::Error::Email));
                    }
                    Ok(true)
                }));
            }
        }
        WellKnown::Hostname(v) => {
            if v {
                fns.push(Arc::new(move |val, _| {
                    let val = val.unwrap_or("".to_string());
                    if let Err(_) = val.validate_hostname() {
                        return Err(Error::new(name.to_string(), string::Error::Hostname));
                    }
                    Ok(true)
                }));
            }
        }
        WellKnown::Ip(v) => {
            if v {
                fns.push(Arc::new(move |val, _| {
                    match val.unwrap_or("".to_string()).validate_ip() {
                        Ok(_) => Ok(true),
                        Err(_) => Err(Error::new(name.to_string(), string::Error::Ip)),
                    }
                }));
            }
        }
        WellKnown::Ipv4(v) => {
            if v {
                fns.push(Arc::new(move |val, _| {
                    match val.unwrap_or("".to_string()).validate_ipv4() {
                        Ok(_) => Ok(true),
                        Err(_) => Err(Error::new(name.to_string(), string::Error::Ipv4)),
                    }
                }));
            }
        }
        WellKnown::Ipv6(v) => {
            if v {
                fns.push(Arc::new(move |val, _| {
                    match val.unwrap_or("".to_string()).validate_ipv6() {
                        Ok(_) => Ok(true),
                        Err(_) => Err(Error::new(name.to_string(), string::Error::Ipv6)),
                    }
                }));
            }
        }
        WellKnown::Uri(v) => {
            if v {
                fns.push(Arc::new(move |val, _| {
                    match val.unwrap_or("".to_string()).validate_uri() {
                        Ok(_) => Ok(true),
                        Err(_) => Err(Error::new(name.to_string(), string::Error::Uri)),
                    }
                }));
            }
        }
        WellKnown::UriRef(v) => {
            if v {
                fns.push(Arc::new(move |val, _| {
                    match val.unwrap_or("".to_string()).validate_uri_ref() {
                        Ok(_) => Ok(true),
                        Err(_) => Err(Error::new(name.to_string(), string::Error::UriRef)),
                    }
                }));
            }
        }
        WellKnown::Address(v) => {
            if v {
                fns.push(Arc::new(move |val, _| {
                    let val = val.unwrap_or("".to_string());
                    if let Ok(()) = val.validate_ip() {
                        return Ok(true);
                    }
                    match val.validate_hostname() {
                        Ok(()) => Ok(true),
                        Err(_) => Err(Error::new(name.to_string(), string::Error::Address)),
                    }
                }));
            }
        }
        WellKnown::Uuid(v) => {
            if v {
                fns.push(Arc::new(move |val, _| {
                    match val.unwrap_or("".to_string()).validate_uuid() {
                        Ok(_) => Ok(true),
                        Err(_) => Err(Error::new(name.to_string(), string::Error::Uuid)),
                    }
                }));
            }
        }
        WellKnown::WellKnownRegex(v) => match KnownRegex::try_from(v) {
            Ok(KnownRegex::HttpHeaderName) => {
                fns.push(Arc::new(move |val, _| {
                    match val.unwrap_or("".to_string()).validate_header_name(strict) {
                        Ok(_) => Ok(true),
                        Err(_) => Err(Error::new(name.to_string(), string::Error::HttpHeaderName)),
                    }
                }));
            }
            Ok(KnownRegex::HttpHeaderValue) => {
                fns.push(Arc::new(move |val, _| {
                    match val.unwrap_or("".to_string()).validate_header_value(strict) {
                        Ok(_) => Ok(true),
                        Err(_) => Err(Error::new(name.to_string(), string::Error::HttpHeaderValue)),
                    }
                }));
            }
            _ => {}
        },
    }
    fns
}
