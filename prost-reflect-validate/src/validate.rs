use crate::validate_proto::DESCRIPTOR_POOL;
use anyhow::{anyhow};
use once_cell::sync::Lazy;
use prost_reflect::{ExtensionDescriptor, Value};
use std::borrow::Cow;

pub(crate) static VALIDATION_DISABLED: Lazy<ExtensionDescriptor>
= Lazy::new(|| DESCRIPTOR_POOL.get_extension_by_name("validate.disabled").ok_or(anyhow!("validate.disabled extension not found")).unwrap());
pub(crate) static VALIDATION_IGNORED: Lazy<ExtensionDescriptor>
= Lazy::new(|| DESCRIPTOR_POOL.get_extension_by_name("validate.ignored").ok_or(anyhow!("validate.ignored extension not found")).unwrap());
pub(crate) static VALIDATION_FIELD_RULES: Lazy<ExtensionDescriptor>
= Lazy::new(|| DESCRIPTOR_POOL.get_extension_by_name("validate.rules").ok_or(anyhow!("validate.rules extension not found")).unwrap());
pub(crate) static VALIDATION_ONE_OF_RULES: Lazy<ExtensionDescriptor>
= Lazy::new(|| DESCRIPTOR_POOL.get_extension_by_name("validate.required").ok_or(anyhow!("validate.required extension not found")).unwrap());

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
