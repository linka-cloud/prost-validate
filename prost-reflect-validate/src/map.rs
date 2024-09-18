use crate::field::{make_validate_field};
use crate::registry::{FieldValidationFn, ValidationFn, REGISTRY};
use crate::validate_proto::field_rules::Type;
use crate::validate_proto::MapRules;
use crate::ValidatorExt;
use anyhow::format_err;
use prost_reflect::{FieldDescriptor, Kind, MapKey, Value};
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

macro_rules! list_rules {
    ($rules:ident) => {
        match &$rules.r#type {
            Some(Type::Map(rules)) => rules,
            _ => return Err(format_err!("unexpected map rules")),
        }
    };
}

fn push<F>(fns: &mut Vec<FieldValidationFn<Box<HashMap<MapKey, Value>>>>, name: Arc<String>, f: Arc<F>)
where
    F: Fn(&HashMap<MapKey, Value>, &MapRules, &String) -> anyhow::Result<bool> + Send + Sync + 'static,
{
    let name = name.clone();
    fns.push(Arc::new(move |val, rules| {
        let val = val.unwrap_or(Box::new(HashMap::new()));
        let rules = list_rules!(rules);
        f(&val, rules, &name)
    }))
}

pub(crate) fn make_validate_map(m: &mut HashMap<String, ValidationFn>, field: FieldDescriptor, rules: Box<MapRules>) -> Vec<FieldValidationFn<Box<HashMap<MapKey, Value>>>> {
    let mut fns = Vec::new();
    let name = Arc::new(field.full_name().to_string());
    // let vals = vals.unwrap_or(&default);
    if rules.ignore_empty() {
        push(&mut fns, name.clone(), Arc::new(move |vals: &HashMap<MapKey, Value>, _: &MapRules, _: &String| {
            Ok(!vals.is_empty())
        }));
    }
    if rules.min_pairs.is_some() {
        push(&mut fns, name.clone(), Arc::new(move |vals: &HashMap<MapKey, Value>, rules: &MapRules, name: &String| {
            let v = rules.min_pairs();
            if vals.len() < v as usize {
                return Err(format_err!("{}: must have at least {} pairs", name, v));
            }
            Ok(true)
        }));
    }
    if rules.max_pairs.is_some() {
        push(&mut fns, name.clone(), Arc::new(move |vals: &HashMap<MapKey, Value>, rules: &MapRules, name: &String| {
            let v = rules.max_pairs();
            if vals.len() > v as usize {
                return Err(format_err!("{}: must have at most {} pairs", name, v));
            }
            Ok(true)
        }));
    }

    let (key_desc, val_desc) = {
        let k = field.kind();
        let k = k.as_message().unwrap();
        (k.get_field(1).unwrap(), k.get_field(2).unwrap())
    };
    if let Some(rules) = rules.keys {
        let validate = make_validate_field(m, &key_desc, &rules);
        push(&mut fns, name.clone(), Arc::new(move |vals: &HashMap<MapKey, Value>, rules: &MapRules, _: &String| {
            let rules = rules.keys.as_ref().unwrap();
            for (k, _) in vals {
                let val = Value::from(k.clone());
                if !validate(Cow::Borrowed(&val), rules)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }));
    }
    if let Some(rules) = rules.values {
        let validate = make_validate_field(m, &val_desc, &rules);
        push(&mut fns, name.clone(), Arc::new(move |vals: &HashMap<MapKey, Value>, rules: &MapRules, _: &String| {
            let rules = rules.values.as_ref().unwrap();
            for (_, val) in vals {
                let val = Value::from(val.clone());
                if !validate(Cow::Borrowed(&val), rules)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }));
    }
    if rules.no_sparse.unwrap_or(false) {
        let kind = Arc::new(field.kind());
        push(&mut fns, name.clone(), Arc::new(move |vals: &HashMap<MapKey, Value>, _: &MapRules, name: &String| {
            let kind = kind.clone();
            for (_, val) in vals {
                if val.is_default(&kind) {
                    return Err(format_err!("{}: must not have sparse values", name));
                }
            }
            Ok(true)
        }));
    }
    if let Kind::Message(desc) = field.kind() {
        if REGISTRY.register(m, &desc).is_err() {
            return fns;
        }
        push(&mut fns, name.clone(), Arc::new(move |vals: &HashMap<MapKey, Value>, _: &MapRules, _: &String| {
            for (_, val) in vals {
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
