use crate::registry::FieldValidationFn;
use prost_validate::format_err;
use prost_reflect::FieldDescriptor;
use prost_validate_types::field_rules::Type;
use prost_validate_types::FieldRules;
use std::sync::Arc;

pub(crate) fn make_validate_bool(
    field: &FieldDescriptor,
    rules: &FieldRules,
) -> Vec<FieldValidationFn<bool>> {
    let mut fns: Vec<FieldValidationFn<bool>> = Vec::new();
    if !matches!(rules.r#type, Some(Type::Bool(_))) {
        return fns;
    }
    let rules = match rules.r#type {
        Some(Type::Bool(rules)) => rules,
        _ => return fns,
    };
    if let Some(v) = rules.r#const {
        let name = field.full_name().to_string();
        fns.push(Arc::new(move |val, _| {
            let val = val.unwrap_or(false);
            if val != v {
                return Err(format_err!(name, "must be {}", v));
            }
            Ok(true)
        }))
    }
    fns
}
