use crate::registry::FieldValidationFn;
use crate::validate_proto::field_rules::Type;
use crate::validate_proto::string_rules::WellKnown;
use crate::validate_proto::{FieldRules, KnownRegex, StringRules};
use anyhow::format_err;
use http::uri::Uri;
use idna::domain_to_ascii;
use once_cell::sync::Lazy;
use prost_reflect::FieldDescriptor;
use regex::Regex;
use std::net::IpAddr;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;

pub(crate) static UUID_RE: Lazy<Regex> = Lazy::new(|| Regex::new(
    r"^[0-9a-fA-F]{8}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{12}$"
).unwrap());

// Regex from the specs
// https://html.spec.whatwg.org/multipage/forms.html#valid-e-mail-address
// It will mark esoteric email addresses like quoted string as invalid
static EMAIL_USER_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+\z").unwrap());
static EMAIL_DOMAIN_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"^[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"
    ).unwrap()
});
// literal form, ipv4 or ipv6 address (SMTP 4.1.3)
static EMAIL_LITERAL_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\[([a-fA-F0-9:\.]+)\]\z").unwrap());

static HEADER_NAME_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^:?[0-9a-zA-Z!#$%&'*+-.^_|~`]+$").unwrap());
static HEADER_VALUE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[^\x00-\b\n-\x1f\x7f]*$").unwrap());

pub(crate) fn validate_domain_part(domain_part: &str) -> bool {
    if EMAIL_DOMAIN_RE.is_match(domain_part) {
        return true;
    }

    // maybe we have an ip as a domain?
    match EMAIL_LITERAL_RE.captures(domain_part) {
        Some(caps) => match caps.get(1) {
            Some(c) => c.as_str().validate_ip(),
            None => false,
        },
        None => false,
    }
}

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

/// Validates whether the given string is an email based on the [HTML5 spec](https://html.spec.whatwg.org/multipage/forms.html#valid-e-mail-address).
/// [RFC 5322](https://tools.ietf.org/html/rfc5322) is not practical in most circumstances and allows email addresses
/// that are unfamiliar to most users.
pub(crate) fn validate_email(val: &str) -> bool {
    if val.is_empty() || !val.contains('@') {
        return false;
    }

    let parts: Vec<&str> = val.rsplitn(2, '@').collect();
    let user_part = parts[1];
    let domain_part = parts[0];

    // prost-reflect-validate the length of each part of the email, BEFORE doing the regex
    // according to RFC5321 the max length of the local part is 64 characters
    // and the max length of the domain part is 255 characters
    // https://datatracker.ietf.org/doc/html/rfc5321#section-4.5.3.1.1
    if user_part.chars().count() > 64 || domain_part.chars().count() > 255 {
        return false;
    }

    if !EMAIL_USER_RE.is_match(user_part) {
        return false;
    }

    if !validate_domain_part(domain_part) {
        // Still the possibility of an [IDN](https://en.wikipedia.org/wiki/Internationalized_domain_name)
        return match domain_to_ascii(domain_part) {
            Ok(d) => validate_domain_part(&d),
            Err(_) => false,
        };
    }

    true
}

fn validate_uri(val: &str) -> bool {
    let uri = match Uri::from_str(val) {
        Ok(uri) => uri,
        Err(_) => return false,
    };
    match uri.scheme() {
        Some(_) => true,
        None => false
    }
}

fn validate_uri_ref(val: &str) -> bool {
    match Uri::from_str(val) {
        Ok(_) => true,
        Err(_) => false,
    }
}

macro_rules! string_rules {
    ($rules:ident) => {
        match &$rules.r#type {
            Some(Type::String(rules)) => rules,
            _ => return Err(format_err!("unexpected string rules")),
        }
    };
}

fn push<F>(fns: &mut Vec<FieldValidationFn<String>>, name: Arc<String>, f: Arc<F>)
where
    F: Fn(String, &StringRules, &String) -> anyhow::Result<bool> + Send + Sync + 'static,
{
    let name = name.clone();
    fns.push(Arc::new(move |val, rules| {
        let val = val.unwrap_or("".to_string());
        let rules = string_rules!(rules);
        f(val, rules, &name)
    }))
}

pub(crate) fn make_validate_string(field: &FieldDescriptor, rules: &FieldRules) -> Vec<FieldValidationFn<String>> {
    let mut fns: Vec<FieldValidationFn<String>> = Vec::new();
    if !matches!(rules.r#type, Some(Type::String(_))) {
        return fns;
    }
    let rules = match &rules.r#type {
        Some(Type::String(rule)) => rule,
        _ => return fns,
    };
    if rules.ignore_empty() {
        fns.push(Arc::new(|val, _| Ok(val.is_some() && val.unwrap() != "")));
    }
    let name = Arc::new(field.full_name().to_string());
    if rules.r#const.is_some() {
        push(&mut fns, name.clone(), Arc::new(move |val: String, rules: &StringRules, name: &String| {
            let v = rules.r#const();
            if val != v {
                return Err(format_err!("{}: must be {}", name, v));
            }
            Ok(true)
        }));
    }
    if rules.len.is_some() {
        push(&mut fns, name.clone(), Arc::new(move |val: String, rules: &StringRules, name: &String| {
            let v = rules.len();
            if val.len() != v as usize {
                return Err(format_err!("{}: must be {} characters long", name, v));
            }
            Ok(true)
        }))
    }
    if rules.min_len.is_some() {
        push(&mut fns, name.clone(), Arc::new(move |val: String, rules: &StringRules, name: &String| {
            let v = rules.min_len();
            if val.len() < v as usize {
                return Err(format_err!("{}: must be minimum {} characters long", name, v));
            }
            Ok(true)
        }));
    }
    if rules.max_len.is_some() {
        push(&mut fns, name.clone(), Arc::new(move |val: String, rules: &StringRules, name: &String| {
            let v = rules.max_len();
            if val.len() > v as usize {
                return Err(format_err!("{}: must be maximum {} characters long", name, v));
            }
            Ok(true)
        }));
    }
    if rules.min_bytes.is_some() {
        push(&mut fns, name.clone(), Arc::new(move |val: String, rules: &StringRules, name: &String| {
            let bytes = val.as_bytes();
            let v = rules.min_bytes();
            if bytes.len() < v as usize {
                return Err(format_err!("{}: must be minimum {} bytes long", name, v));
            }
            Ok(true)
        }));
    }
    if rules.max_bytes.is_some() {
        push(&mut fns, name.clone(), Arc::new(move |val: String, rules: &StringRules, name: &String| {
            let bytes = val.as_bytes();
            let v = rules.max_bytes();
            if bytes.len() > v as usize {
                return Err(format_err!("{}: must be maximum {} bytes long", name, v));
            }
            Ok(true)
        }));
    }
    if let Some(v) = &rules.pattern {
        let regex = Regex::new(v.as_str()).unwrap();
        push(&mut fns, name.clone(), Arc::new(move |val: String, rules: &StringRules, name: &String| {
            let v = rules.pattern();
            if !regex.is_match(val.as_str()) {
                return Err(format_err!("{}: must matches {}", name, v));
            }
            Ok(true)
        }));
    }
    if rules.prefix.is_some() {
        push(&mut fns, name.clone(), Arc::new(move |val: String, rules: &StringRules, name: &String| {
            let v = rules.prefix();
            if !val.as_str().starts_with(v) {
                return Err(format_err!("{}: must have prefix {}", name, v));
            }
            Ok(true)
        }));
    }
    if rules.suffix.is_some() {
        push(&mut fns, name.clone(), Arc::new(move |val: String, rules: &StringRules, name: &String| {
            let v = rules.suffix();
            if !val.as_str().ends_with(v) {
                return Err(format_err!("{}: must have suffix {}", name, v));
            }
            Ok(true)
        }));
    }
    if rules.contains.is_some() {
        push(&mut fns, name.clone(), Arc::new(move |val: String, rules: &StringRules, name: &String| {
            let v = rules.contains();
            if !val.contains(v) {
                return Err(format_err!("{}: must contains {}", name, v));
            }
            Ok(true)
        }));
    }
    if rules.not_contains.is_some() {
        push(&mut fns, name.clone(), Arc::new(move |val: String, rules: &StringRules, name: &String| {
            let v = rules.not_contains();
            if val.contains(v) {
                return Err(format_err!("{}: must not contains {}", name, v));
            }
            Ok(true)
        }));
    }
    if !rules.r#in.is_empty() {
        push(&mut fns, name.clone(), Arc::new(move |val: String, rules: &StringRules, name: &String| {
            let v = rules.r#in.deref();
            if !v.contains(&val.to_string()) {
                return Err(format_err!("{}: must be in {:?}", name, v));
            }
            Ok(true)
        }));
    }
    if !rules.not_in.is_empty() {
        push(&mut fns, name.clone(), Arc::new(move |val: String, rules: &StringRules, name: &String| {
            let v = rules.not_in.deref();
            if v.contains(&val.to_string()) {
                return Err(format_err!("{}: must not be in {:?}", name, v));
            }
            Ok(true)
        }));
    }
    if rules.well_known.is_none() {
        return fns;
    }
    match rules.well_known.unwrap() {
        WellKnown::Email(v) => {
            if v {
                fns.push(Arc::new(move |val, _| {
                    let val = val.unwrap_or("".to_string());
                    if !validate_email(val.as_str()) {
                        return Err(format_err!("{}: must be a valid email", name));
                    }
                    Ok(true)
                }));
            }
        }
        WellKnown::Hostname(v) => {
            if v {
                fns.push(Arc::new(move |val, _| {
                    let val = val.unwrap_or("".to_string());
                    if !validate_domain_part(val.as_str()) {
                        return Err(format_err!("{}: must be a valid hostname", name));
                    }
                    Ok(true)
                }));
            }
        }
        WellKnown::Ip(v) => {
            if v {
                fns.push(Arc::new(move |val, _| {
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
                fns.push(Arc::new(move |val, _| {
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
                fns.push(Arc::new(move |val, _| {
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
                fns.push(Arc::new(move |val, _| {
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
                fns.push(Arc::new(move |val, _| {
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
                fns.push(Arc::new(move |val, _| {
                    let val = val.unwrap_or("".to_string());
                    if !validate_domain_part(val.as_str()) && !val.validate_ip() {
                        return Err(format_err!("{}: must be a valid hostname or ip address", name));
                    }
                    Ok(true)
                }));
            }
        }
        WellKnown::Uuid(v) => {
            if v {
                fns.push(Arc::new(move |val, _| {
                    let val = val.unwrap_or("".to_string());
                    if !UUID_RE.is_match(val.as_str()) {
                        return Err(format_err!("{}: must be a valid uuid", name));
                    }
                    Ok(true)
                }));
            }
        }
        WellKnown::WellKnownRegex(v) => {
            match KnownRegex::try_from(v) {
                Ok(KnownRegex::HttpHeaderName) => {
                    fns.push(Arc::new(move |val, _| {
                        let val = val.unwrap_or("".to_string());
                        if !HEADER_NAME_RE.is_match(val.as_str()) {
                            return Err(format_err!("{}: must match regex pattern \"^:?[0-9a-zA-Z!#$%&'*+-.^_|~`]+$\"", name));
                        }
                        Ok(true)
                    }));
                }
                Ok(KnownRegex::HttpHeaderValue) => {
                    fns.push(Arc::new(move |val, _| {
                        let val = val.unwrap_or("".to_string());
                        if !HEADER_VALUE_RE.is_match(val.as_str()) {
                            return Err(format_err!("{}: must match regex pattern \"^[^\\x00-\\b\\n-\\x1f\\x7f]*$\"", name));
                        }
                        Ok(true)
                    }));
                }
                _ => {}
            }
        }
    }
    fns
}
