use crate::registry::FieldValidationFn;
use crate::validate_proto::field_rules::Type;
use crate::validate_proto::FieldRules;
use anyhow::format_err;
use prost_reflect::FieldDescriptor;
use std::sync::Arc;

pub(crate) fn make_validate_bool(field: &FieldDescriptor, rules: &FieldRules) -> Vec<FieldValidationFn<bool>> {
    let mut fns: Vec<FieldValidationFn<bool>> = Vec::new();
    if !matches!(rules.r#type, Some(Type::Bool(_))) {
        return fns;
    }
    let rules = match rules.r#type {
        Some(Type::Bool(rules)) => rules,
        _ => return fns,
    };
    if rules.r#const.is_some() {
        let name = field.full_name().to_string();
        fns.push(Arc::new(move |val, _| {
            let val = val.unwrap_or(false);
            if rules.r#const.unwrap() != val {
                return Err(format_err!("{}: must be {}", name, rules.r#const.unwrap()));
            }
            Ok(true)
        }))
    }
    fns
}
