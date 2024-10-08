use crate::registry::FieldValidationFn;
use prost_validate::format_err;
use prost_reflect::FieldDescriptor;
use prost_validate::ValidateString;
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
                    return Err(format_err!(name, "must be {v}"));
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
                let v = rules.len();
                if val.chars().count() != v as usize {
                    return Err(format_err!(name, "must be {v} characters long"));
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
                let v = rules.min_len();
                if val.chars().count() < v as usize {
                    return Err(format_err!(name, "must be minimum {v} characters long"));
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
                let v = rules.max_len();
                if val.chars().count() > v as usize {
                    return Err(format_err!(name, "must be maximum {v} characters long"));
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
                let v = rules.len_bytes();
                if val.len() != v as usize {
                    return Err(format_err!(name, "must be {v} characters long"));
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
                let v = rules.min_bytes();
                if val.len() < v as usize {
                    return Err(format_err!(name, "must be minimum {v} bytes long"));
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
                let v = rules.max_bytes();
                if val.len() > v as usize {
                    return Err(format_err!(name, "must be maximum {v} bytes long"));
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
                    Err(err) => return Err(format_err!(name, "invalid regex pattern: {err}")),
                };
                if !regex.is_match(val.as_str()) {
                    return Err(format_err!(name, "must matches {v}"));
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
                    return Err(format_err!(name, "must have prefix {v}"));
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
                    return Err(format_err!(name, "must have suffix {v}"));
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
                    return Err(format_err!(name, "must contains {v}"));
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
                    return Err(format_err!(name, "must not contains {v}"));
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
                    return Err(format_err!(name, "must be in {:?}", v));
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
                    return Err(format_err!(name, "must not be in {:?}", v));
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
                    if let Err(err) = val.validate_email() {
                        return Err(format_err!(name, "{err}"));
                    }
                    Ok(true)
                }));
            }
        }
        WellKnown::Hostname(v) => {
            if v {
                fns.push(Arc::new(move |val, _| {
                    let val = val.unwrap_or("".to_string());
                    if let Err(err) = val.validate_hostname() {
                        return Err(format_err!(name, "{err}"));
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
                        Err(_) => Err(format_err!(name, "must be a valid ip")),
                    }
                }));
            }
        }
        WellKnown::Ipv4(v) => {
            if v {
                fns.push(Arc::new(move |val, _| {
                    match val.unwrap_or("".to_string()).validate_ipv4() {
                        Ok(_) => Ok(true),
                        Err(_) => Err(format_err!(name, "must be a valid ipv4")),
                    }
                }));
            }
        }
        WellKnown::Ipv6(v) => {
            if v {
                fns.push(Arc::new(move |val, _| {
                    match val.unwrap_or("".to_string()).validate_ipv6() {
                        Ok(_) => Ok(true),
                        Err(_) => Err(format_err!(name, "must be a valid ipv6")),
                    }
                }));
            }
        }
        WellKnown::Uri(v) => {
            if v {
                fns.push(Arc::new(move |val, _| {
                    match val.unwrap_or("".to_string()).validate_uri() {
                        Ok(_) => Ok(true),
                        Err(_) => Err(format_err!(name, "must be a valid URI")),
                    }
                }));
            }
        }
        WellKnown::UriRef(v) => {
            if v {
                fns.push(Arc::new(move |val, _| {
                    match val.unwrap_or("".to_string()).validate_uri_ref() {
                        Ok(_) => Ok(true),
                        Err(_) => Err(format_err!(name, "must be a valid URI")),
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
                        Err(_) => Err(format_err!(
                            name, "must be a valid hostname or ip address"
                        )),
                    }
                }));
            }
        }
        WellKnown::Uuid(v) => {
            if v {
                fns.push(Arc::new(move |val, _| {
                    match val.unwrap_or("".to_string()).validate_uuid() {
                        Ok(_) => Ok(true),
                        Err(_) => Err(format_err!(name, "must be a valid uuid")),
                    }
                }));
            }
        }
        WellKnown::WellKnownRegex(v) => match KnownRegex::try_from(v) {
            Ok(KnownRegex::HttpHeaderName) => {
                fns.push(Arc::new(move |val, _| {
                    match val.unwrap_or("".to_string()).validate_header_name(strict) {
                        Ok(_) => Ok(true),
                        Err(err) => Err(format_err!(name, "{err}")),
                    }
                }));
            }
            Ok(KnownRegex::HttpHeaderValue) => {
                fns.push(Arc::new(move |val, _| {
                    match val.unwrap_or("".to_string()).validate_header_value(strict) {
                        Ok(_) => Ok(true),
                        Err(err) => Err(format_err!(name, "{err}")),
                    }
                }));
            }
            _ => {}
        },
    }
    fns
}
