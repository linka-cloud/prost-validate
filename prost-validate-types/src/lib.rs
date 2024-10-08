#[allow(clippy::len_without_is_empty)]
mod proto;

use anyhow::anyhow;
use once_cell::sync::Lazy;
use prost_reflect::{
    ExtensionDescriptor, FieldDescriptor, MessageDescriptor, OneofDescriptor, Value,
};
use std::borrow::Cow;
pub use proto::*;

#[allow(clippy::unwrap_used)]
static VALIDATION_DISABLED: Lazy<ExtensionDescriptor> = Lazy::new(|| {
    DESCRIPTOR_POOL
        .get_extension_by_name("validate.disabled")
        .ok_or(anyhow!("validate.disabled extension not found"))
        .unwrap()
});
#[allow(clippy::unwrap_used)]
static VALIDATION_IGNORED: Lazy<ExtensionDescriptor> = Lazy::new(|| {
    DESCRIPTOR_POOL
        .get_extension_by_name("validate.ignored")
        .ok_or(anyhow!("validate.ignored extension not found"))
        .unwrap()
});
#[allow(clippy::unwrap_used)]
static VALIDATION_FIELD_RULES: Lazy<ExtensionDescriptor> = Lazy::new(|| {
    DESCRIPTOR_POOL
        .get_extension_by_name("validate.rules")
        .ok_or(anyhow!("validate.rules extension not found"))
        .unwrap()
});
#[allow(clippy::unwrap_used)]
static VALIDATION_ONE_OF_RULES: Lazy<ExtensionDescriptor> = Lazy::new(|| {
    DESCRIPTOR_POOL
        .get_extension_by_name("validate.required")
        .ok_or(anyhow!("validate.required extension not found"))
        .unwrap()
});

pub trait FieldRulesExt {
    fn validation_rules(&self) -> anyhow::Result<Option<FieldRules>>;
}

impl FieldRulesExt for FieldDescriptor {
    fn validation_rules(&self) -> anyhow::Result<Option<FieldRules>> {
        match self
            .options()
            .get_extension(&VALIDATION_FIELD_RULES)
            .as_message()
        {
            Some(r) => Ok(Some(r.transcode_to::<FieldRules>()?)),
            None => Ok(None),
        }
    }
}

pub trait OneofRulesExt {
    fn required(&self) -> bool;
}

impl OneofRulesExt for OneofDescriptor {
    fn required(&self) -> bool {
        self.options()
            .get_extension(&VALIDATION_ONE_OF_RULES)
            .is_true()
    }
}

pub trait MessageRulesExt {
    fn validation_disabled(&self) -> bool;
    fn validation_ignored(&self) -> bool;
}

impl MessageRulesExt for MessageDescriptor {
    fn validation_disabled(&self) -> bool {
        self.options().get_extension(&VALIDATION_DISABLED).is_true()
    }

    fn validation_ignored(&self) -> bool {
        self.options().get_extension(&VALIDATION_IGNORED).is_true()
    }
}

trait IsTrueExt {
    fn is_true(&self) -> bool;
}

impl<'a> IsTrueExt for Cow<'a, Value> {
    fn is_true(&self) -> bool {
        self.as_bool().unwrap_or(false)
    }
}
