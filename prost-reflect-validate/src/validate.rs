use crate::field::validate_field;
use crate::list::validate_list;
use crate::map::validate_map;
use crate::validate_proto::field_rules::Type;
use crate::validate_proto::FieldRules;
use crate::validate_proto::DESCRIPTOR_POOL;
use anyhow::{anyhow, format_err, Result};
use once_cell::sync::Lazy;
use prost_reflect::{ExtensionDescriptor, ReflectMessage, Value};
use std::borrow::Cow;

pub(crate) static VALIDATION_DISABLED: Lazy<ExtensionDescriptor>
= Lazy::new(|| DESCRIPTOR_POOL.get_extension_by_name("validate.disabled").ok_or(anyhow!("validate.disabled extension not found")).unwrap());
pub(crate) static VALIDATION_IGNORED: Lazy<ExtensionDescriptor>
= Lazy::new(|| DESCRIPTOR_POOL.get_extension_by_name("validate.ignored").ok_or(anyhow!("validate.ignored extension not found")).unwrap());
pub(crate) static VALIDATION_FIELD_RULES: Lazy<ExtensionDescriptor>
= Lazy::new(|| DESCRIPTOR_POOL.get_extension_by_name("validate.rules").ok_or(anyhow!("validate.rules extension not found")).unwrap());
pub(crate) static VALIDATION_ONE_OF_RULES: Lazy<ExtensionDescriptor>
= Lazy::new(|| DESCRIPTOR_POOL.get_extension_by_name("validate.required").ok_or(anyhow!("validate.required extension not found")).unwrap());

pub(crate) fn validate(msg: &impl ReflectMessage) -> Result<()> {
    let msg = msg.transcode_to_dynamic();
    let opts = msg.descriptor().options();
    if opts.get_extension(&VALIDATION_DISABLED).is_true() || opts.get_extension(&VALIDATION_IGNORED).is_true() {
        return Ok(());
    }
    for field in msg.descriptor().fields() {
        let opts = field.options();
        if field.containing_oneof().is_some_and(|v| v.options().get_extension(&VALIDATION_ONE_OF_RULES).is_true()) {
            let mut has = false;
            for field in field.containing_oneof().unwrap().fields() {
                let desc = msg.descriptor().get_field(field.number()).ok_or(format_err!("oneof field {} not found", field.name()))?;
                let is_default = msg.get_field(&field).is_default(&desc.kind());
                if has && !is_default {
                    return Err(format_err!("oneof {} contains multiple values", field.containing_oneof().unwrap().name()));
                }
                has = !is_default;
            }
            if !has {
                return Err(format_err!("oneof {} does not contains any value", field.containing_oneof().unwrap().name()));
            }
        }
        let rules = opts.get_extension(&VALIDATION_FIELD_RULES);
        let rules = match rules.as_message() {
            Some(r) => r,
            None => continue,
        };
        let rules = rules.transcode_to::<FieldRules>()?;
        if rules.message.is_none() && rules.r#type.is_none() {
            continue;
        }
        // println!("{}: {:#?})", field.full_name(), rules);
        let val = msg.get_field(&field);
        if field.is_list() {
            let vals = val.as_list();
            if let Some(Type::Repeated(rules)) = rules.r#type {
                validate_list(vals, &field, rules)?;
            }
            continue;
        }
        if field.is_map() {
            let vals = val.as_map();
            if let Some(Type::Map(rules)) = rules.clone().r#type {
                validate_map(vals, &field, rules)?;
            }
            continue;
        }
        validate_field(val, &field, &rules)?;
    }
    Ok(())
}

pub(crate) trait IsTrue {
    fn is_true(&self) -> bool;
}

impl<'a> IsTrue for Cow<'a, Value> {
    fn is_true(&self) -> bool {
        match self.as_bool() {
            Some(true) => true,
            _ => false,
        }
    }
}
