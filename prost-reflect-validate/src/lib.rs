use crate::registry::REGISTRY;
use prost_reflect::ReflectMessage;

mod number;
mod string;
mod r#enum;
mod duration;
mod timestamp;
mod message;
mod bool;
mod any;
mod utils;
mod bytes;
mod registry;
mod validate;
mod list;
mod map;
mod field;

mod validate_proto {
    use once_cell::sync::Lazy;
    use prost_reflect::DescriptorPool;

    pub(crate) static DESCRIPTOR_POOL: Lazy<DescriptorPool>
    = Lazy::new(|| DescriptorPool::decode(include_bytes!(concat!(env!("OUT_DIR"), "/file_descriptor_set.bin")).as_ref()).unwrap());
    include!(concat!(env!("OUT_DIR"), "/validate.rs"));
}

pub trait ValidatorExt {
    fn validate(&self) -> anyhow::Result<()>;
}

impl<T: ReflectMessage> ValidatorExt for T {
    fn validate(&self) -> anyhow::Result<()> {
        let msg = self.transcode_to_dynamic();
        REGISTRY.validate(&msg)
    }
}
