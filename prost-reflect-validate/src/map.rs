use crate::field::make_validate_field;
use crate::registry::{NestedValidationFn, ValidationFn, REGISTRY};
use prost_reflect::{FieldDescriptor, Kind, MapKey, Value};
use prost_validate::errors::map;
use prost_validate::{format_err, Error};
use prost_validate_types::field_rules::Type;
use prost_validate_types::{FieldRules, MapRules};
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

fn push<F>(fns: &mut Vec<NestedValidationFn<HashMap<MapKey, Value>>>, name: &Arc<String>, f: Arc<F>)
where
    F: Fn(
            &HashMap<MapKey, Value>,
            &MapRules,
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
pub(crate) fn make_validate_map(
    m: &mut HashMap<String, ValidationFn>,
    field: &FieldDescriptor,
    rules: &FieldRules,
) -> Vec<NestedValidationFn<HashMap<MapKey, Value>>> {
    let mut fns = Vec::new();
    let name = Arc::new(field.full_name().to_string());
    let (key_desc, val_desc) = {
        let k = field.kind();
        let k = k.as_message().unwrap();
        (k.get_field(1).unwrap(), k.get_field(2).unwrap())
    };
    if let Some(Type::Map(rules)) = &rules.r#type {
        if rules.ignore_empty() {
            push(
                &mut fns,
                &name,
                Arc::new(
                    move |vals: &HashMap<MapKey, Value>,
                          _: &MapRules,
                          _: &String,
                          _: &HashMap<String, ValidationFn>| {
                        Ok(!vals.is_empty())
                    },
                ),
            );
        }
        if rules.min_pairs.is_some() {
            push(
                &mut fns,
                &name,
                Arc::new(
                    move |vals: &HashMap<MapKey, Value>,
                          rules: &MapRules,
                          name: &String,
                          _: &HashMap<String, ValidationFn>| {
                        let v = rules.min_pairs();
                        if vals.len() < v as usize {
                            return Err(Error::new(
                                name.to_string(),
                                map::Error::MinPairs(v as usize),
                            ));
                        }
                        Ok(true)
                    },
                ),
            );
        }
        if rules.max_pairs.is_some() {
            push(
                &mut fns,
                &name,
                Arc::new(
                    move |vals: &HashMap<MapKey, Value>,
                          rules: &MapRules,
                          name: &String,
                          _: &HashMap<String, ValidationFn>| {
                        let v = rules.max_pairs();
                        if vals.len() > v as usize {
                            return Err(Error::new(
                                name.to_string(),
                                map::Error::MaxPairs(v as usize),
                            ));
                        }
                        Ok(true)
                    },
                ),
            );
        }
        if let Some(ref rules) = rules.keys {
            let validate = make_validate_field(m, &key_desc, rules);
            push(
                &mut fns,
                &name,
                Arc::new(
                    move |vals: &HashMap<MapKey, Value>,
                          rules: &MapRules,
                          name: &String,
                          m: &HashMap<String, ValidationFn>| {
                        let rules = rules.keys.as_ref().unwrap();
                        for k in vals.keys() {
                            let val = Value::from(k.clone());
                            if !validate(Cow::Borrowed(&val), rules, m).map_err(|e| {
                                Error::new(
                                    format!("{}[{:?}]", name, map_key_string(k)),
                                    map::Error::Keys(Box::new(e)),
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
        if let Some(ref rules) = rules.values {
            if rules.message.map(|v| v.skip()).unwrap_or(false) {
                return fns;
            }
            let validate = make_validate_field(m, &val_desc, rules);
            push(
                &mut fns,
                &name,
                Arc::new(
                    move |vals: &HashMap<MapKey, Value>,
                          rules: &MapRules,
                          name: &String,
                          m: &HashMap<String, ValidationFn>| {
                        let rules = rules.values.as_ref().unwrap();
                        for (k, val) in vals.iter() {
                            let val = val.clone();
                            if !validate(Cow::Borrowed(&val), rules, m).map_err(|e| {
                                Error::new(
                                    format!("{}[{:?}]", name, map_key_string(k)),
                                    map::Error::Values(Box::new(e)),
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
        if rules.no_sparse.unwrap_or(false) {
            let kind = Arc::new(field.kind());
            push(
                &mut fns,
                &name,
                Arc::new(
                    move |vals: &HashMap<MapKey, Value>,
                          _: &MapRules,
                          name: &String,
                          _: &HashMap<String, ValidationFn>| {
                        let kind = kind.clone();
                        for (k, val) in vals.iter() {
                            if val.is_default(&kind) {
                                return Err(Error::new(
                                    format!("{}[{:?}]", name, map_key_string(k)),
                                    map::Error::NoSparse,
                                ));
                            }
                        }
                        Ok(true)
                    },
                ),
            );
        }
    }
    if let Kind::Message(desc) = val_desc.kind() {
        if REGISTRY.register(m, &desc).is_err() {
            return fns;
        }
        fns.push(Arc::new(move |vals, _, m| {
            if let Some(vals) = vals {
                for (k, val) in vals.iter() {
                    if let Some(Err(err)) = val.as_message().map(|v| REGISTRY.do_validate(v, m)) {
                        return Err(Error::new(
                            format!("{}[{:?}]", name, map_key_string(k)),
                            map::Error::Values(Box::new(err)),
                        ));
                    }
                }
            }
            Ok(true)
        }));
    }
    fns
}

fn map_key_string(k: &MapKey) -> String {
    match k {
        MapKey::Bool(k) => k.to_string(),
        MapKey::I32(k) => k.to_string(),
        MapKey::I64(k) => k.to_string(),
        MapKey::U32(k) => k.to_string(),
        MapKey::U64(k) => k.to_string(),
        MapKey::String(k) => k.to_string(),
    }
}
