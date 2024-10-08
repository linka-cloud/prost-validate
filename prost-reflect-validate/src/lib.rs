#![doc = include_str!("../README.md")]

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

/// Extension trait for validating messages using [`prost-reflect`](https://docs.rs/prost-reflect/latest/prost_reflect/).
///
/// The implementation is provided for the [`prost_reflect::ReflectMessage`](https://docs.rs/prost-reflect/latest/prost_reflect/trait.ReflectMessage.html) trait:
/// ```rust
///  use prost_reflect_validate::ValidatorExt;
///  use example_proto::ExampleMessage;
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
    fn validate(&self) -> prost_validate::Result<()>;
}

impl<T: ReflectMessage> ValidatorExt for T {
    fn validate(&self) -> prost_validate::Result<()> {
        validate(self)
    }
}

pub fn validate<T: ReflectMessage>(msg: &T) -> prost_validate::Result<()> {
    let msg = msg.transcode_to_dynamic();
    REGISTRY.validate(&msg)
}
