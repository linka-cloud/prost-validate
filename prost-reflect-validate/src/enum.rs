use crate::registry::FieldValidationFn;
use prost_reflect::{FieldDescriptor, Kind};
use prost_validate::errors::r#enum;
use prost_validate::{format_err, Error, Result};
use prost_validate_types::field_rules::Type;
use prost_validate_types::{EnumRules, FieldRules};
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

pub(crate) fn make_validate_enum(
    field: &FieldDescriptor,
    rules: &FieldRules,
) -> Vec<FieldValidationFn<i32>> {
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
        push(
            &mut fns,
            &name,
            Arc::new(move |val: i32, _: &EnumRules, name: &String| {
                if val != v {
                    return Err(Error::new(name.to_string(), r#enum::Error::Const(v)));
                }
                Ok(true)
            }),
        );
    }
    if !rules.r#in.is_empty() {
        push(
            &mut fns,
            &name,
            Arc::new(move |val: i32, rules: &EnumRules, name: &String| {
                if !rules.r#in.contains(&val) {
                    return Err(Error::new(
                        name.to_string(),
                        r#enum::Error::In(rules.r#in.clone()),
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
            Arc::new(move |val: i32, rules: &EnumRules, name: &String| {
                if rules.not_in.contains(&val) {
                    return Err(Error::new(
                        name.to_string(),
                        r#enum::Error::NotIn(rules.not_in.clone()),
                    ));
                }
                Ok(true)
            }),
        );
    }
    if rules.defined_only() {
        push(
            &mut fns,
            &name,
            Arc::new(move |val: i32, _: &EnumRules, name: &String| {
                if desc.get_value(val).is_none() {
                    return Err(Error::new(name.to_string(), r#enum::Error::DefinedOnly));
                }
                Ok(true)
            }),
        );
    }
    fns
}
