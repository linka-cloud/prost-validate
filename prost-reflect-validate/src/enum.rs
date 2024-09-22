use crate::registry::FieldValidationFn;
use crate::validate_proto::field_rules::Type;
use crate::validate_proto::{EnumRules, FieldRules};
use anyhow::{format_err, Result};
use prost_reflect::{FieldDescriptor, Kind};
use std::sync::Arc;

macro_rules! enum_rules {
    ($rules:ident) => {
        match &$rules.r#type {
            Some(Type::Enum(rules)) => rules,
            _ => return Err(format_err!("unexpected enum rules")),
        }
    };
}

fn push<F>(fns: &mut Vec<FieldValidationFn<i32>>, name: &Arc<String>, f: Arc<F>)
where
    F: Fn(i32, &EnumRules, &String) -> Result<bool> + Send + Sync + 'static,
{
    let name = name.clone();
    fns.push(Arc::new(move |val, rules| {
        let val = val.unwrap_or(0);
        let rules = enum_rules!(rules);
        f(val, rules, &name)
    }))
}

pub(crate) fn make_validate_enum(field: &FieldDescriptor, rules: &FieldRules) -> Vec<FieldValidationFn<i32>> {
    let mut fns: Vec<FieldValidationFn<i32>> = Vec::new();
    let name = Arc::new(field.full_name().to_string());
    if !matches!(rules.r#type, Some(Type::Enum(_))) {
        return Vec::new();
    }
    let desc = match field.kind() {
        Kind::Enum(d) => d,
        _ => return Vec::new(),
    };
    let rules = match &rules.r#type {
        Some(Type::Enum(rules)) => rules,
        _ => return Vec::new(),
    };
    if let Some(v) = rules.r#const {
        push(&mut fns, &name, Arc::new(move |val: i32, _: &EnumRules, name: &String| {
            if val != v {
                return Err(format_err!("{}: must be {}", name, v));
            }
            Ok(true)
        }));
    }
    if !rules.r#in.is_empty() {
        push(&mut fns, &name, Arc::new(move |val: i32, rules: &EnumRules, name: &String| {
            if !rules.r#in.contains(&val) {
                return Err(format_err!("{}: must be in {:?}", name, rules.r#in));
            }
            Ok(true)
        }));
    }
    if !rules.not_in.is_empty() {
        push(&mut fns, &name, Arc::new(move |val: i32, rules: &EnumRules, name: &String| {
            if rules.not_in.contains(&val) {
                return Err(format_err!("{}: must not be in {:?}", name, rules.not_in));
            }
            Ok(true)
        }));
    }
    if rules.defined_only() {
        push(&mut fns, &name, Arc::new(move |val: i32, _: &EnumRules, name: &String| {
            if desc.get_value(val).is_none() {
                return Err(format_err!("{}: must be a defined enumeration value", name));
            }
            Ok(true)
        }));
    }
    fns
}
