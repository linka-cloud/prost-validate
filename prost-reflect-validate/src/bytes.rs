use crate::registry::FieldValidationFn;
use crate::validate_proto::bytes_rules::WellKnown;
use crate::validate_proto::field_rules::Type;
use crate::validate_proto::{BytesRules, FieldRules};
use anyhow::format_err;
use prost::bytes::Bytes;
use prost_reflect::FieldDescriptor;
use regex::bytes::Regex;
use std::ops::Deref;
use std::sync::Arc;

pub(crate) fn validate_bytes(val: Option<&Bytes>, field: &FieldDescriptor, rules: &FieldRules) -> anyhow::Result<()> {
    if !matches!(rules.r#type, Some(Type::Bytes(_))) {
        return Ok(());
    }
    let default = Bytes::default();
    let val = val.unwrap_or(&default);
    let rules = match &rules.r#type {
        Some(Type::Bytes(rule)) => rule,
        _ => return Ok(()),
    };
    if rules.ignore_empty() && val == "" {
        return Ok(());
    }
    if let Some(v) = &rules.r#const {
        if v != val {
            return Err(format_err!("{}: must be {:?}", field.full_name(), v));
        }
    }
    if let Some(l) = rules.len {
        if val.len() != l as usize {
            return Err(format_err!("{}: must be {} characters long", field.full_name(), l));
        }
    }
    if let Some(m) = rules.min_len {
        if val.len() < m as usize {
            return Err(format_err!("{}: must be minimum {} characters long", field.full_name(), m));
        }
    }
    if let Some(m) = rules.max_len {
        if val.len() > m as usize {
            return Err(format_err!("{}: must be maximum {} characters long", field.full_name(), m));
        }
    }
    if let Some(p) = &rules.pattern {
        let regex = Regex::new(p)?;
        if !regex.is_match(val.iter().as_slice()) {
            return Err(format_err!("{}: must matches {}", field.full_name(), p));
        }
    }
    if let Some(p) = &rules.prefix {
        if !val.starts_with(p) {
            return Err(format_err!("{}: must have prefix {:?}", field.full_name(), p));
        }
    }
    if let Some(s) = &rules.suffix {
        if !val.ends_with(s) {
            return Err(format_err!("{}: must have suffix {:?}", field.full_name(), s));
        }
    }
    if let Some(v) = &rules.contains {
        let v = Bytes::from(v.clone());
        if !contains_slice(val, &v) {
            return Err(format_err!("{}: must contains {:?}", field.full_name(), v));
        }
    }
    if !rules.r#in.is_empty() && !rules.r#in.contains(&val.deref().into()) {
        return Err(format_err!("{}: must be in {:?}", field.full_name(), rules.r#in));
    }
    if !rules.not_in.is_empty() && !rules.not_in.contains(&val.deref().into()) {
        return Err(format_err!("{}: must not be in {:?}", field.full_name(), rules.not_in));
    }
    if rules.well_known.is_none() {
        return Ok(());
    }
    match rules.well_known.unwrap() {
        WellKnown::Ip(v) => {
            if v && (val.len() == 16 || val.len() == 8) {
                return Err(format_err!("{}: must be a valid ip", field.full_name()));
            }
        }
        WellKnown::Ipv4(v) => {
            if v && val.len() == 8 {
                return Err(format_err!("{}: must be a valid ipv4", field.full_name()));
            }
        }
        WellKnown::Ipv6(v) => {
            if v && val.len() == 16 {
                return Err(format_err!("{}: must be a valid ipv6", field.full_name()));
            }
        }
    }
    Ok(())
}

fn push<F>(fns: &mut Vec<FieldValidationFn<Arc<Bytes>>>, name: Arc<String>, f: Arc<F>)
where
    F: Fn(&Bytes, &BytesRules, &String) -> anyhow::Result<bool> + Send + Sync + 'static,
{
    let name = name.clone();
    fns.push(Arc::new(move |val, rules| {
        let default = Arc::new(Bytes::default());
        let val = val.unwrap_or(default);
        let rules = match &rules.r#type {
            Some(Type::Bytes(rules)) => rules,
            _ => return Err(format_err!("unexpected string rules")),
        };
        f(&val, rules, &name)
    }))
}

pub(crate) fn make_validate_bytes(field: &FieldDescriptor, rules: &FieldRules) -> Vec<FieldValidationFn<Arc<Bytes>>> {
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
        push(&mut fns, name.clone(), Arc::new(move |val: &Bytes, _: &BytesRules, _: &String| {
            Ok(val != "")
        }));
    }
    if rules.r#const.is_some() {
        push(&mut fns, name.clone(), Arc::new(move |val: &Bytes, rules: &BytesRules, name: &String| {
            if val != rules.r#const() {
                return Err(format_err!("{}: must be {:?}", name, rules.r#const()));
            }
            Ok(true)
        }));
    }
    if rules.len.is_some() {
        push(&mut fns, name.clone(), Arc::new(move |val: &Bytes, rules: &BytesRules, name: &String| {
            if val.len() != rules.len() as usize {
                return Err(format_err!("{}: must be {} characters long", name, rules.len()));
            }
            Ok(true)
        }));
    }
    if rules.min_len.is_some() {
        push(&mut fns, name.clone(), Arc::new(move |val: &Bytes, rules: &BytesRules, name: &String| {
            if val.len() < rules.min_len() as usize {
                return Err(format_err!("{}: must be minimum {} characters long", name, rules.min_len()));
            }
            Ok(true)
        }));
    }
    if rules.max_len.is_some() {
        push(&mut fns, name.clone(), Arc::new(move |val: &Bytes, rules: &BytesRules, name: &String| {
            if val.len() > rules.max_len() as usize {
                return Err(format_err!("{}: must be maximum {} characters long", name, rules.max_len()));
            }
            Ok(true)
        }));
    }
    if rules.pattern.is_some() {
        push(&mut fns, name.clone(), Arc::new(move |val: &Bytes, rules: &BytesRules, name: &String| {
            let regex = Regex::new(rules.pattern())?;
            if !regex.is_match(val.iter().as_slice()) {
                return Err(format_err!("{}: must matches {}", name, rules.pattern()));
            }
            Ok(true)
        }));
    }
    if rules.prefix.is_some() {
        push(&mut fns, name.clone(), Arc::new(move |val: &Bytes, rules: &BytesRules, name: &String| {
            if !val.starts_with(rules.prefix()) {
                return Err(format_err!("{}: must have prefix {:?}", name, rules.prefix()));
            }
            Ok(true)
        }));
    }
    if rules.suffix.is_some() {
        push(&mut fns, name.clone(), Arc::new(move |val: &Bytes, rules: &BytesRules, name: &String| {
            if !val.ends_with(rules.suffix()) {
                return Err(format_err!("{}: must have suffix {:?}", name, rules.suffix()));
            }
            Ok(true)
        }));
    }
    if rules.contains.is_some() {
        push(&mut fns, name.clone(), Arc::new(move |val: &Bytes, rules: &BytesRules, name: &String| {
            let v = Bytes::from(rules.contains().to_vec());
            if !contains_slice(val, &v) {
                return Err(format_err!("{}: must contains {:?}", name, v));
            }
            Ok(true)
        }));
    }
    if !rules.r#in.is_empty() {
        push(&mut fns, name.clone(), Arc::new(move |val: &Bytes, rules: &BytesRules, name: &String| {
            if !rules.r#in.contains(&val.deref().into()) {
                return Err(format_err!("{}: must be in {:?}", name, rules.r#in));
            }
            Ok(true)
        }));
    }
    if !rules.not_in.is_empty() {
        push(&mut fns, name.clone(), Arc::new(move |val: &Bytes, rules: &BytesRules, name: &String| {
            if !rules.not_in.contains(&val.deref().into()) {
                return Err(format_err!("{}: must not be in {:?}", name, rules.not_in));
            }
            Ok(true)
        }));
    }
    if rules.well_known.is_none() {
        return fns;
    }
    match rules.well_known.unwrap() {
        WellKnown::Ip(v) => {
            if v {
                push(&mut fns, name.clone(), Arc::new(move |val: &Bytes, _: &BytesRules, name: &String| {
                    if val.len() == 16 || val.len() == 8 {
                        return Err(format_err!("{}: must be a valid ip", name));
                    }
                    Ok(true)
                }));
            }
        }
        WellKnown::Ipv4(v) => {
            if v {
                push(&mut fns, name.clone(), Arc::new(move |val: &Bytes, _: &BytesRules, name: &String| {
                    if val.len() == 8 {
                        return Err(format_err!("{}: must be a valid ipv4", name));
                    }
                    Ok(true)
                }));
            }
        }
        WellKnown::Ipv6(v) => {
            if v {
                push(&mut fns, name.clone(), Arc::new(move |val: &Bytes, _: &BytesRules, name: &String| {
                    if val.len() == 16 {
                        return Err(format_err!("{}: must be a valid ipv6", name));
                    }
                    Ok(true)
                }));
            }
        }
    }
    fns
}

fn contains_slice(slice: &'_ Bytes, sub: &'_ Bytes) -> bool
{
    let len = sub.len();
    if len == 0 {
        return true;
    }
    slice.windows(len)
        .any(move |sub_slice| sub_slice == sub)
}
