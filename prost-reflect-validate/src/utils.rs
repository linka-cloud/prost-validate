use prost_reflect::{FieldDescriptor, Value};
use prost_validate_types::{FieldRules, FieldRulesExt};
use std::borrow::Cow;
use std::sync::Arc;

#[allow(clippy::ptr_arg)]
pub(crate) fn is_set(val: &Cow<Value>) -> bool {
    match val {
        Cow::Borrowed(_) => true,
        Cow::Owned(_) => false,
    }
}

pub(crate) fn get_field_rules(field: &FieldDescriptor) -> anyhow::Result<Option<Arc<FieldRules>>> {
    let rules = match field.validation_rules()? {
        Some(r) => r,
        None => return Ok(None),
    };
    Ok(Some(Arc::new(rules)))
}
