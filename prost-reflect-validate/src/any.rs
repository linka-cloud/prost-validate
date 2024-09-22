use crate::registry::NestedValidationFn;
use crate::validate_proto::field_rules::Type;
use crate::validate_proto::{AnyRules, FieldRules};
use anyhow::format_err;
use prost_reflect::{DynamicMessage, FieldDescriptor};
use prost_types::Any;
use std::sync::Arc;


macro_rules! any_rules {
    ($rules:ident) => {
        match &$rules.r#type {
            Some(Type::Any(rules)) => rules,
            _ => return Err(format_err!("unexpected any rules")),
        }
    };
}

fn push<F>(fns: &mut Vec<NestedValidationFn<Box<DynamicMessage>>>, name: &Arc<String>, f: Arc<F>)
where
    F: Fn(&Any, &AnyRules, &String) -> anyhow::Result<bool> + Send + Sync + 'static,
{
    let name = name.clone();
    fns.push(Arc::new(move |val, rules, _| {
        let val = match val {
            Some(v) => v.transcode_to::<Any>()?,
            None => Any::default(),
        };
        let rules = any_rules!(rules);
        f(&val, rules, &name)
    }))
}

pub(crate) fn make_validate_any(field: &FieldDescriptor, rules: &FieldRules) -> Vec<NestedValidationFn<Box<DynamicMessage>>> {
    let mut fns = Vec::new();
    if !matches!(rules.r#type, Some(Type::Any(_))) {
        return fns;
    }
    let rules = match &rules.r#type {
        Some(Type::Any(rules)) => rules,
        _ => return fns,
    };
    let name = Arc::new(field.full_name().to_string());
    if rules.required() {
        let name = name.clone();
        fns.push(Arc::new(move |val, _, _| {
            if val.is_none() {
                return Err(format_err!("{}: is required", name));
            }
            Ok(true)
        }));
    } else {
        fns.push(Arc::new(move |val, _, _| {
            Ok(val.is_some())
        }));
    }
    if !rules.r#in.is_empty() {
        push(&mut fns, &name, Arc::new(move |val: &Any, rules: &AnyRules, name: &String| {
            if !rules.r#in.contains(&val.type_url) {
                return Err(format_err!("{}: must be in {:?}", name, rules.r#in));
            }
            Ok(true)
        }));
    }
    if !rules.not_in.is_empty() {
        push(&mut fns, &name, Arc::new(move |val: &Any, rules: &AnyRules, name: &String| {
            if rules.not_in.contains(&val.type_url) {
                return Err(format_err!("{}: must not be in {:?}", name, rules.not_in));
            }
            Ok(true)
        }));
    }
    fns
}
