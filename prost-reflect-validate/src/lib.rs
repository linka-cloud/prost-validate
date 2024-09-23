#![doc = include_str!("../README.md")]
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

mod any;
mod bool;
mod bytes;
mod duration;
mod r#enum;
mod field;
mod list;
mod map;
mod message;
mod number;
mod registry;
mod string;
mod timestamp;
mod utils;
mod validate;

#[allow(clippy::trivially_copy_pass_by_ref)]
#[allow(clippy::enum_variant_names)]
mod validate_proto {
    use once_cell::sync::Lazy;
    use prost_reflect::DescriptorPool;

    #[allow(clippy::unwrap_used)]
    pub(crate) static DESCRIPTOR_POOL: Lazy<DescriptorPool> = Lazy::new(|| {
        DescriptorPool::decode(
            include_bytes!(concat!(env!("OUT_DIR"), "/file_descriptor_set.bin")).as_ref(),
        )
        .unwrap()
    });
    include!(concat!(env!("OUT_DIR"), "/validate.rs"));
}

/// Extension trait for validating messages using `prost-reflect`.
///
/// The implementation is provided for the `prost_reflect::ReflectMessage` trait:
/// ```rust
///  use prost_reflect_validate::ValidatorExt;
///  use crate::proto::ExampleMessage;
///
///  match ExampleMessage::default().validate() {
///     Ok(_) => println!("Validation passed"),
///     Err(e) => eprintln!("Validation failed: {}", e),
///  }
///  let msg = ExampleMessage{content: "Hello, world!".to_string()};
///  match msg.validate() {
///     Ok(_) => println!("Validation passed"),
///     Err(e) => eprintln!("Validation failed: {}", e),
///  }
/// ```
pub trait ValidatorExt: Send + Sync {
    fn validate(&self) -> anyhow::Result<()>;
}

impl<T: ReflectMessage> ValidatorExt for T {
    fn validate(&self) -> anyhow::Result<()> {
        let msg = self.transcode_to_dynamic();
        REGISTRY.validate(&msg)
    }
}
