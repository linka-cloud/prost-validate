use crate::registry::FieldValidationFn;
use crate::validate_proto::field_rules::Type;
use crate::validate_proto::FieldRules;
use anyhow::format_err;
use prost_reflect::FieldDescriptor;

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
        fns.push(Box::new(move |val, _| {
            let val = val.unwrap_or(false);
            if val != v {
                return Err(format_err!("{}: must be {}", name, v));
            }
            Ok(true)
        }))
    }
    fns
}
