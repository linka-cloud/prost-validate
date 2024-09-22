use crate::registry::FieldValidationFn;
use crate::validate_proto::field_rules::Type;
use crate::validate_proto::string_rules::WellKnown;
use crate::validate_proto::{FieldRules, KnownRegex, StringRules};
use anyhow::{format_err, Result};
use email_address::EmailAddress;
use http::uri::Uri;
use once_cell::sync::Lazy;
use prost_reflect::FieldDescriptor;
use regex::Regex;
use std::net::IpAddr;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;

fn validate_hostname(host: &str) -> Result<()> {
    let host = host.trim_end_matches('.').to_lowercase();
    if host.len() > 253 {
        return Err(format_err!("hostname cannot exceed 253 characters"));
    }
    for part in host.split('.') {
        let l = part.len();
        if l == 0 || l > 63 {
            return Err(format_err!(
                "hostname part must be non-empty and cannot exceed 63 characters"
            ));
        }
        if part.starts_with('-') {
            return Err(format_err!("hostname parts cannot begin with hyphens"));
        }
        if part.ends_with('-') {
            return Err(format_err!("hostname parts cannot end with hyphens"));
        }
        for r in part.chars() {
            if !(r.is_ascii_alphanumeric() || r == '-') {
                return Err(format_err!(
                    "hostname parts can only contain alphanumeric characters or hyphens, got {}",
                    r
                ));
            }
        }
    }
    Ok(())
}

fn validate_email(addr: &str) -> Result<()> {
    EmailAddress::from_str(addr)?;
    Ok(())
}

#[allow(clippy::unwrap_used)]
pub(crate) static UUID_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"^[0-9a-fA-F]{8}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{12}$",
    )
    .unwrap()
});

#[allow(clippy::unwrap_used)]
static HEADER_NAME_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^:?[0-9a-zA-Z!#$%&'*+-.^_|~`]+$").unwrap());
#[allow(clippy::unwrap_used)]
static HEADER_VALUE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[^\x00-\x08\x0A-\x1F\x7F]*$").unwrap());
#[allow(clippy::unwrap_used)]
static HEADER_STRING_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[^\x00\n\r]*$").unwrap());

pub(crate) trait ValidateIp {
    /// Validates whether the given string is an IP V4
    fn validate_ipv4(&self) -> bool;
    /// Validates whether the given string is an IP V6
    fn validate_ipv6(&self) -> bool;
    /// Validates whether the given string is an IP
    fn validate_ip(&self) -> bool;
}

impl<T> ValidateIp for T
where
    T: ToString,
{
    fn validate_ipv4(&self) -> bool {
        IpAddr::from_str(&self.to_string()).map_or(false, |i| i.is_ipv4())
    }

    fn validate_ipv6(&self) -> bool {
        IpAddr::from_str(&self.to_string()).map_or(false, |i| i.is_ipv6())
    }

    fn validate_ip(&self) -> bool {
        IpAddr::from_str(&self.to_string()).is_ok()
    }
}

fn validate_uri(val: &str) -> bool {
    let uri = match Uri::from_str(val) {
        Ok(uri) => uri,
        Err(_) => return false,
    };
    uri.scheme().is_some()
}

fn validate_uri_ref(val: &str) -> bool {
    Uri::from_str(val).is_ok()
}

macro_rules! string_rules {
    ($rules:ident) => {
        match &$rules.r#type {
            Some(Type::String(rules)) => rules,
            _ => return Err(format_err!("unexpected string rules")),
        }
    };
}

fn push<F>(fns: &mut Vec<FieldValidationFn<String>>, name: &Arc<String>, f: Box<F>)
where
    F: Fn(String, &StringRules, &String) -> anyhow::Result<bool> + Send + Sync + 'static,
{
    let name = name.clone();
    fns.push(Box::new(move |val, rules| {
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
        fns.push(Box::new(|val, _| {
            Ok(val.map(|v| !v.is_empty()).unwrap_or(false))
        }));
    }
    let name = Arc::new(field.full_name().to_string());
    if rules.r#const.is_some() {
        push(
            &mut fns,
            &name,
            Box::new(move |val: String, rules: &StringRules, name: &String| {
                let v = rules.r#const();
                if val != v {
                    return Err(format_err!("{}: must be {}", name, v));
                }
                Ok(true)
            }),
        );
    }
    if rules.len.is_some() {
        push(
            &mut fns,
            &name,
            Box::new(move |val: String, rules: &StringRules, name: &String| {
                let v = rules.len();
                if val.chars().count() != v as usize {
                    return Err(format_err!("{}: must be {} characters long", name, v));
                }
                Ok(true)
            }),
        )
    }
    if rules.min_len.is_some() {
        push(
            &mut fns,
            &name,
            Box::new(move |val: String, rules: &StringRules, name: &String| {
                let v = rules.min_len();
                if val.chars().count() < v as usize {
                    return Err(format_err!(
                        "{}: must be minimum {} characters long",
                        name,
                        v
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
            Box::new(move |val: String, rules: &StringRules, name: &String| {
                let v = rules.max_len();
                if val.chars().count() > v as usize {
                    return Err(format_err!(
                        "{}: must be maximum {} characters long",
                        name,
                        v
                    ));
                }
                Ok(true)
            }),
        );
    }
    if rules.len_bytes.is_some() {
        push(
            &mut fns,
            &name,
            Box::new(move |val: String, rules: &StringRules, name: &String| {
                let v = rules.len_bytes();
                if val.len() != v as usize {
                    return Err(format_err!("{}: must be {} characters long", name, v));
                }
                Ok(true)
            }),
        )
    }
    if rules.min_bytes.is_some() {
        push(
            &mut fns,
            &name,
            Box::new(move |val: String, rules: &StringRules, name: &String| {
                let v = rules.min_bytes();
                if val.len() < v as usize {
                    return Err(format_err!("{}: must be minimum {} bytes long", name, v));
                }
                Ok(true)
            }),
        );
    }
    if rules.max_bytes.is_some() {
        push(
            &mut fns,
            &name,
            Box::new(move |val: String, rules: &StringRules, name: &String| {
                let v = rules.max_bytes();
                if val.len() > v as usize {
                    return Err(format_err!("{}: must be maximum {} bytes long", name, v));
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
            Box::new(move |val: String, rules: &StringRules, name: &String| {
                let v = rules.pattern();
                let regex = match &regex {
                    Ok(r) => r,
                    Err(err) => {
                        return Err(format_err!("{}: invalid regex pattern: {}", name, err))
                    }
                };
                if !regex.is_match(val.as_str()) {
                    return Err(format_err!("{}: must matches {}", name, v));
                }
                Ok(true)
            }),
        );
    }
    if rules.prefix.is_some() {
        push(
            &mut fns,
            &name,
            Box::new(move |val: String, rules: &StringRules, name: &String| {
                let v = rules.prefix();
                if !val.as_str().starts_with(v) {
                    return Err(format_err!("{}: must have prefix {}", name, v));
                }
                Ok(true)
            }),
        );
    }
    if rules.suffix.is_some() {
        push(
            &mut fns,
            &name,
            Box::new(move |val: String, rules: &StringRules, name: &String| {
                let v = rules.suffix();
                if !val.as_str().ends_with(v) {
                    return Err(format_err!("{}: must have suffix {}", name, v));
                }
                Ok(true)
            }),
        );
    }
    if rules.contains.is_some() {
        push(
            &mut fns,
            &name,
            Box::new(move |val: String, rules: &StringRules, name: &String| {
                let v = rules.contains();
                if !val.contains(v) {
                    return Err(format_err!("{}: must contains {}", name, v));
                }
                Ok(true)
            }),
        );
    }
    if rules.not_contains.is_some() {
        push(
            &mut fns,
            &name,
            Box::new(move |val: String, rules: &StringRules, name: &String| {
                let v = rules.not_contains();
                if val.contains(v) {
                    return Err(format_err!("{}: must not contains {}", name, v));
                }
                Ok(true)
            }),
        );
    }
    if !rules.r#in.is_empty() {
        push(
            &mut fns,
            &name,
            Box::new(move |val: String, rules: &StringRules, name: &String| {
                let v = rules.r#in.deref();
                if !v.contains(&val.to_string()) {
                    return Err(format_err!("{}: must be in {:?}", name, v));
                }
                Ok(true)
            }),
        );
    }
    if !rules.not_in.is_empty() {
        push(
            &mut fns,
            &name,
            Box::new(move |val: String, rules: &StringRules, name: &String| {
                let v = rules.not_in.deref();
                if v.contains(&val.to_string()) {
                    return Err(format_err!("{}: must not be in {:?}", name, v));
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
                fns.push(Box::new(move |val, _| {
                    let val = val.unwrap_or("".to_string());
                    if let Err(err) = validate_email(val.as_str()) {
                        return Err(format_err!("{}: {}", name, err));
                    }
                    Ok(true)
                }));
            }
        }
        WellKnown::Hostname(v) => {
            if v {
                fns.push(Box::new(move |val, _| {
                    let val = val.unwrap_or("".to_string());
                    if let Err(err) = validate_hostname(val.as_str()) {
                        return Err(format_err!("{}: {}", name, err));
                    }
                    Ok(true)
                }));
            }
        }
        WellKnown::Ip(v) => {
            if v {
                fns.push(Box::new(move |val, _| {
                    let val = val.unwrap_or("".to_string());
                    if !val.validate_ip() {
                        return Err(format_err!("{}: must be a valid ip", name));
                    }
                    Ok(true)
                }));
            }
        }
        WellKnown::Ipv4(v) => {
            if v {
                fns.push(Box::new(move |val, _| {
                    let val = val.unwrap_or("".to_string());
                    if !val.validate_ipv4() {
                        return Err(format_err!("{}: must be a valid ipv4", name));
                    }
                    Ok(true)
                }));
            }
        }
        WellKnown::Ipv6(v) => {
            if v {
                fns.push(Box::new(move |val, _| {
                    let val = val.unwrap_or("".to_string());
                    if !val.validate_ipv6() {
                        return Err(format_err!("{}: must be a valid ipv6", name));
                    }
                    Ok(true)
                }));
            }
        }
        WellKnown::Uri(v) => {
            if v {
                fns.push(Box::new(move |val, _| {
                    let val = val.unwrap_or("".to_string());
                    if !validate_uri(val.as_str()) {
                        return Err(format_err!("{}: must be a valid absolute URI", name));
                    }
                    Ok(true)
                }));
            }
        }
        WellKnown::UriRef(v) => {
            if v {
                fns.push(Box::new(move |val, _| {
                    let val = val.unwrap_or("".to_string());
                    if !validate_uri_ref(val.as_str()) {
                        return Err(format_err!("{}: must be a valid URI", name));
                    }
                    Ok(true)
                }));
            }
        }
        WellKnown::Address(v) => {
            if v {
                fns.push(Box::new(move |val, _| {
                    let val = val.unwrap_or("".to_string());
                    if val.validate_ip() {
                        return Ok(true);
                    }
                    if validate_hostname(val.as_str()).is_err() {
                        return Err(format_err!(
                            "{}: must be a valid hostname or ip address",
                            name
                        ));
                    }
                    Ok(true)
                }));
            }
        }
        WellKnown::Uuid(v) => {
            if v {
                fns.push(Box::new(move |val, _| {
                    let val = val.unwrap_or("".to_string());
                    if !UUID_RE.is_match(val.as_str()) {
                        return Err(format_err!("{}: must be a valid uuid", name));
                    }
                    Ok(true)
                }));
            }
        }
        WellKnown::WellKnownRegex(v) => match KnownRegex::try_from(v) {
            Ok(KnownRegex::HttpHeaderName) => {
                fns.push(Box::new(move |val, _| {
                    let val = val.unwrap_or("".to_string());
                    let mut regex = &HEADER_NAME_RE;
                    if !strict {
                        regex = &HEADER_STRING_RE;
                    }
                    if !regex.is_match(val.as_str()) {
                        return Err(format_err!(
                            "{}: must match regex pattern \"^:?[0-9a-zA-Z!#$%&'*+-.^_|~`]+$\"",
                            name
                        ));
                    }
                    Ok(true)
                }));
            }
            Ok(KnownRegex::HttpHeaderValue) => {
                fns.push(Box::new(move |val, _| {
                    let val = val.unwrap_or("".to_string());
                    let mut regex = &HEADER_VALUE_RE;
                    if !strict {
                        regex = &HEADER_STRING_RE;
                    }
                    if !regex.is_match(val.as_str()) {
                        return Err(format_err!(
                            "{}: must match regex pattern \"^[^\\x00-\\b\\n-\\x1f\\x7f]*$\"",
                            name
                        ));
                    }
                    Ok(true)
                }));
            }
            _ => {}
        },
    }
    fns
}
