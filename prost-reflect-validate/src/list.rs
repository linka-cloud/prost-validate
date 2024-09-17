use crate::field::{make_validate_field};
use crate::registry::{FieldValidationFn, ValidationFn, REGISTRY};
use crate::validate_proto::field_rules::Type;
use crate::validate_proto::RepeatedRules;
use crate::ValidatorExt;
use anyhow::format_err;
use itertools::Itertools;
use prost_reflect::bytes::Bytes;
use prost_reflect::{FieldDescriptor, Kind, Value};
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

macro_rules! list_rules {
    ($rules:ident) => {
        match &$rules.r#type {
            Some(Type::Repeated(rules)) => rules,
            _ => return Err(format_err!("unexpected list rules")),
        }
    };
}

fn push<F>(fns: &mut Vec<FieldValidationFn<Box<Vec<Value>>>>, name: Arc<String>, f: Arc<F>)
where
    F: Fn(&[Value], &RepeatedRules, &String) -> anyhow::Result<bool> + Send + Sync + 'static,
{
    let name = name.clone();
    fns.push(Arc::new(move |val, rules| {
        let val = val.unwrap_or(Box::new(Vec::new()));
        let rules = list_rules!(rules);
        f(&val, rules, &name)
    }))
}

pub(crate) fn make_validate_list(m: &mut HashMap<String, ValidationFn>, field: FieldDescriptor, rules: Box<RepeatedRules>) -> Vec<FieldValidationFn<Box<Vec<Value>>>> {
    let mut fns = Vec::new();
    let name = Arc::new(field.full_name().to_string());
    if rules.ignore_empty() {
        push(&mut fns, name.clone(), Arc::new(move |vals: &[Value], _: &RepeatedRules, _: &String| {
            Ok(!vals.is_empty())
        }));
    }
    if let Some(_) = rules.min_items {
        push(&mut fns, name.clone(), Arc::new(move |vals: &[Value], rules: &RepeatedRules, name: &String| {
            let v = rules.min_items.unwrap();
            if vals.len() < v as usize {
                return Err(format_err!("{}: must have at least {} items", name, v));
            }
            Ok(true)
        }));
    }
    if let Some(_) = rules.max_items {
        push(&mut fns, name.clone(), Arc::new(move |vals: &[Value], rules: &RepeatedRules, name: &String| {
            let v = rules.max_items.unwrap();
            if vals.len() > v as usize {
                return Err(format_err!("{}: must have at most {} items", name, v));
            }
            Ok(true)
        }));
    }
    if rules.unique.unwrap_or(false) {
        let field = field.clone();
        push(&mut fns, name.clone(), Arc::new(move |vals: &[Value], _: &RepeatedRules, name: &String| {
            if let Some(v) = unique_count(vals, &field) {
                if vals.len() != v {
                    return Err(format_err!("{}: must have unique values", name));
                }
            }
            Ok(true)
        }));
    }
    if let Some(rules) = rules.items {
        let validate = make_validate_field(m, &field, &rules);
        push(&mut fns, name.clone(), Arc::new(move |vals: &[Value], rules: &RepeatedRules, _: &String| {
            let rules = rules.items.as_ref().unwrap();
            for val in vals {
                if !validate(Cow::Borrowed(val), rules)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }));
    }
    if let Kind::Message(desc) = field.kind() {
        if REGISTRY.register(m, &desc).is_err() {
            return fns;
        }
        push(&mut fns, name.clone(), Arc::new(move |vals: &[Value], _: &RepeatedRules, _: &String| {
            for val in vals {
                match val.as_message().map(|v| v.validate()) {
                    Some(Err(err)) => return Err(err),
                    _ => {}
                }
            }
            Ok(true)
        }));
    }
    fns
}

fn unique_count(vals: &[Value], field: &FieldDescriptor) -> Option<usize> {
    match field.kind() {
        Kind::Double => Some(vals.into_iter().map(|v| v.as_f64().unwrap().to_bits()).unique().collect::<Vec<u64>>().len()),
        Kind::Float => Some(vals.into_iter().map(|v| v.as_f32().unwrap().to_bits()).unique().collect::<Vec<u32>>().len()),
        Kind::Int32 => Some(vals.into_iter().map(|v| v.as_i32().unwrap()).unique().collect::<Vec<i32>>().len()),
        Kind::Int64 => Some(vals.into_iter().map(|v| v.as_i64().unwrap()).unique().collect::<Vec<i64>>().len()),
        Kind::Uint32 => Some(vals.into_iter().map(|v| v.as_u32().unwrap()).unique().collect::<Vec<u32>>().len()),
        Kind::Uint64 => Some(vals.into_iter().map(|v| v.as_u64().unwrap()).unique().collect::<Vec<u64>>().len()),
        Kind::Sint32 => Some(vals.into_iter().map(|v| v.as_i32().unwrap()).unique().collect::<Vec<i32>>().len()),
        Kind::Sint64 => Some(vals.into_iter().map(|v| v.as_i64().unwrap()).unique().collect::<Vec<i64>>().len()),
        Kind::Fixed32 => Some(vals.into_iter().map(|v| v.as_u32().unwrap()).unique().collect::<Vec<u32>>().len()),
        Kind::Fixed64 => Some(vals.into_iter().map(|v| v.as_u64().unwrap()).unique().collect::<Vec<u64>>().len()),
        Kind::Sfixed32 => Some(vals.into_iter().map(|v| v.as_i32().unwrap()).unique().collect::<Vec<i32>>().len()),
        Kind::Sfixed64 => Some(vals.into_iter().map(|v| v.as_i64().unwrap()).unique().collect::<Vec<i64>>().len()),
        Kind::Bool => Some(vals.into_iter().map(|v| v.as_bool().unwrap()).unique().collect::<Vec<bool>>().len()),
        Kind::String => Some(vals.into_iter().map(|v| v.as_str().unwrap()).unique().collect::<Vec<&str>>().len()),
        Kind::Bytes => Some(vals.into_iter().map(|v| v.as_bytes().unwrap()).unique().collect::<Vec<&Bytes>>().len()),
        Kind::Message(_) => None,
        Kind::Enum(_) => Some(vals.into_iter().map(|v| v.as_enum_number().unwrap()).unique().collect::<Vec<i32>>().len()),
    }
}
