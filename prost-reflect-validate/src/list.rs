use crate::field::make_validate_field;
use crate::registry::{NestedValidationFn, ValidationFn, REGISTRY};
use itertools::Itertools;
use prost_reflect::bytes::Bytes;
use prost_reflect::{FieldDescriptor, Kind, Value};
use prost_validate::errors::list;
use prost_validate::format_err;
use prost_validate::Error;
use prost_validate_types::field_rules::Type;
use prost_validate_types::{FieldRules, RepeatedRules};
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

fn push<F>(fns: &mut Vec<NestedValidationFn<Vec<Value>>>, name: &Arc<String>, f: Arc<F>)
where
    F: Fn(
            &[Value],
            &RepeatedRules,
            &String,
            &HashMap<String, ValidationFn>,
        ) -> prost_validate::Result<bool>
        + Send
        + Sync
        + 'static,
{
    let name = name.clone();
    fns.push(Arc::new(move |val, rules, m| {
        let val = val.unwrap_or_default();
        let rules = list_rules!(rules);
        f(&val, rules, &name, m)
    }))
}

#[allow(clippy::unwrap_used)]
pub(crate) fn make_validate_list(
    m: &mut HashMap<String, ValidationFn>,
    field: &FieldDescriptor,
    rules: &FieldRules,
) -> Vec<NestedValidationFn<Vec<Value>>> {
    let mut fns = Vec::new();
    let name = Arc::new(field.full_name().to_string());
    if let Some(Type::Repeated(rules)) = &rules.r#type {
        if rules.ignore_empty() {
            push(
                &mut fns,
                &name,
                Arc::new(
                    move |vals: &[Value],
                          _: &RepeatedRules,
                          _: &String,
                          _: &HashMap<String, ValidationFn>| {
                        Ok(!vals.is_empty())
                    },
                ),
            );
        }
        if rules.min_items.is_some() {
            push(
                &mut fns,
                &name,
                Arc::new(
                    move |vals: &[Value],
                          rules: &RepeatedRules,
                          name: &String,
                          _: &HashMap<String, ValidationFn>| {
                        let v = rules.min_items.unwrap();
                        if vals.len() < v as usize {
                            return Err(Error::new(
                                name.to_string(),
                                list::Error::MinItems(v as usize),
                            ));
                        }
                        Ok(true)
                    },
                ),
            );
        }
        if rules.max_items.is_some() {
            push(
                &mut fns,
                &name,
                Arc::new(
                    move |vals: &[Value],
                          rules: &RepeatedRules,
                          name: &String,
                          _: &HashMap<String, ValidationFn>| {
                        let v = rules.max_items.unwrap();
                        if vals.len() > v as usize {
                            return Err(Error::new(
                                name.to_string(),
                                list::Error::MaxItems(v as usize),
                            ));
                        }
                        Ok(true)
                    },
                ),
            );
        }
        if rules.unique.unwrap_or(false) {
            let field = field.clone();
            push(
                &mut fns,
                &name,
                Arc::new(
                    move |vals: &[Value],
                          _: &RepeatedRules,
                          name: &String,
                          _: &HashMap<String, ValidationFn>| {
                        if let Some(v) = unique_count(vals, &field) {
                            if vals.len() != v {
                                return Err(Error::new(name.to_string(), list::Error::Unique));
                            }
                        }
                        Ok(true)
                    },
                ),
            );
        }
        if let Some(ref rules) = rules.items {
            if rules.message.map(|v| v.skip()).unwrap_or(false) {
                return fns;
            }
            let validate = make_validate_field(m, field, rules);
            push(
                &mut fns,
                &name,
                Arc::new(
                    move |vals: &[Value],
                          rules: &RepeatedRules,
                          name: &String,
                          m: &HashMap<String, ValidationFn>| {
                        let rules = rules.items.as_ref().unwrap();
                        for (i, val) in vals.iter().enumerate() {
                            if !validate(Cow::Borrowed(val), rules, m).map_err(|e| {
                                Error::new(
                                    format!("{}[{}]", name, i),
                                    list::Error::Item(Box::new(e)),
                                )
                            })? {
                                return Ok(false);
                            }
                        }
                        Ok(true)
                    },
                ),
            );
        }
    }

    if let Kind::Message(ref desc) = field.kind() {
        if REGISTRY.register(m, desc).is_err() {
            return fns;
        }
        let name = Arc::new(field.full_name().to_string());
        fns.push(Arc::new(move |vals, _, m| {
            if let Some(vals) = vals {
                for (i, val) in vals.iter().enumerate() {
                    if let Some(Err(err)) = val.as_message().map(|v| REGISTRY.do_validate(v, m)) {
                        return Err(Error::new(
                            format!("{}[{}]", name.clone(), i),
                            list::Error::Item(Box::new(err)),
                        ));
                    }
                }
            }
            Ok(true)
        }));
    }
    fns
}

#[allow(clippy::unwrap_used)]
fn unique_count(vals: &[Value], field: &FieldDescriptor) -> Option<usize> {
    match field.kind() {
        Kind::Double => Some(
            vals.iter()
                .map(|v| v.as_f64().unwrap().to_bits())
                .unique()
                .collect::<Vec<u64>>()
                .len(),
        ),
        Kind::Float => Some(
            vals.iter()
                .map(|v| v.as_f32().unwrap().to_bits())
                .unique()
                .collect::<Vec<u32>>()
                .len(),
        ),
        Kind::Int32 => Some(
            vals.iter()
                .map(|v| v.as_i32().unwrap())
                .unique()
                .collect::<Vec<i32>>()
                .len(),
        ),
        Kind::Int64 => Some(
            vals.iter()
                .map(|v| v.as_i64().unwrap())
                .unique()
                .collect::<Vec<i64>>()
                .len(),
        ),
        Kind::Uint32 => Some(
            vals.iter()
                .map(|v| v.as_u32().unwrap())
                .unique()
                .collect::<Vec<u32>>()
                .len(),
        ),
        Kind::Uint64 => Some(
            vals.iter()
                .map(|v| v.as_u64().unwrap())
                .unique()
                .collect::<Vec<u64>>()
                .len(),
        ),
        Kind::Sint32 => Some(
            vals.iter()
                .map(|v| v.as_i32().unwrap())
                .unique()
                .collect::<Vec<i32>>()
                .len(),
        ),
        Kind::Sint64 => Some(
            vals.iter()
                .map(|v| v.as_i64().unwrap())
                .unique()
                .collect::<Vec<i64>>()
                .len(),
        ),
        Kind::Fixed32 => Some(
            vals.iter()
                .map(|v| v.as_u32().unwrap())
                .unique()
                .collect::<Vec<u32>>()
                .len(),
        ),
        Kind::Fixed64 => Some(
            vals.iter()
                .map(|v| v.as_u64().unwrap())
                .unique()
                .collect::<Vec<u64>>()
                .len(),
        ),
        Kind::Sfixed32 => Some(
            vals.iter()
                .map(|v| v.as_i32().unwrap())
                .unique()
                .collect::<Vec<i32>>()
                .len(),
        ),
        Kind::Sfixed64 => Some(
            vals.iter()
                .map(|v| v.as_i64().unwrap())
                .unique()
                .collect::<Vec<i64>>()
                .len(),
        ),
        Kind::Bool => Some(
            vals.iter()
                .map(|v| v.as_bool().unwrap())
                .unique()
                .collect::<Vec<bool>>()
                .len(),
        ),
        Kind::String => Some(
            vals.iter()
                .map(|v| v.as_str().unwrap())
                .unique()
                .collect::<Vec<&str>>()
                .len(),
        ),
        Kind::Bytes => Some(
            vals.iter()
                .map(|v| v.as_bytes().unwrap())
                .unique()
                .collect::<Vec<&Bytes>>()
                .len(),
        ),
        Kind::Message(_) => None,
        Kind::Enum(_) => Some(
            vals.iter()
                .map(|v| v.as_enum_number().unwrap())
                .unique()
                .collect::<Vec<i32>>()
                .len(),
        ),
    }
}
