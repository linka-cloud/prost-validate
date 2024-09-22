#![deny(warnings)]
#![warn(unused_extern_crates)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]
#![warn(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unreachable)]
#![deny(clippy::await_holding_lock)]
#![deny(clippy::needless_pass_by_value)]
#![deny(clippy::trivially_copy_pass_by_ref)]

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

#[allow(clippy::trivially_copy_pass_by_ref)]
#[allow(clippy::enum_variant_names)]
mod validate_proto {
    use once_cell::sync::Lazy;
    use prost_reflect::DescriptorPool;

    #[allow(clippy::unwrap_used)]
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
