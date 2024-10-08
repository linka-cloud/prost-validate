use crate::registry::NestedValidationFn;
use prost_validate::format_err;
use prost_reflect::{DynamicMessage, FieldDescriptor};
use prost_types::Any;
use prost_validate_types::field_rules::Type;
use prost_validate_types::{AnyRules, FieldRules};
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
    F: Fn(&Any, &AnyRules, &String) -> prost_validate::Result<bool> + Send + Sync + 'static,
{
    let name = name.clone();
    fns.push(Arc::new(move |val, rules, _| {
        let val = match val {
            Some(v) => v.transcode_to::<Any>().map_err(|_| format_err!(name, "failed to transcode Any"))?,
            None => Any::default(),
        };
        let rules = any_rules!(rules);
        f(&val, rules, &name)
    }))
}

pub(crate) fn make_validate_any(
    field: &FieldDescriptor,
    rules: &FieldRules,
) -> Vec<NestedValidationFn<Box<DynamicMessage>>> {
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
                return Err(format_err!(name, "is required"));
            }
            Ok(true)
        }));
    } else {
        fns.push(Arc::new(move |val, _, _| Ok(val.is_some())));
    }
    if !rules.r#in.is_empty() {
        push(
            &mut fns,
            &name,
            Arc::new(move |val: &Any, rules: &AnyRules, name: &String| {
                if !rules.r#in.contains(&val.type_url) {
                    return Err(format_err!(name, "must be in {:?}", rules.r#in));
                }
                Ok(true)
            }),
        );
    }
    if !rules.not_in.is_empty() {
        push(
            &mut fns,
            &name,
            Arc::new(move |val: &Any, rules: &AnyRules, name: &String| {
                if rules.not_in.contains(&val.type_url) {
                    return Err(format_err!(name, "must not be in {:?}", rules.not_in));
                }
                Ok(true)
            }),
        );
    }
    fns
}
