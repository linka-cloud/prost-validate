# prost-validate

A protobuf library extending [prost](https://github.com/tokio-rs/prost)
and [prost-reflect](https://github.com/andrewhickman/prost-reflect) with validation support.

This a rust implementation [protoc-gen-validate](https://github.com/bufbuild/protoc-gen-validate).

## Usage

It currently supports validation using reflection.

It must be used with [prost](https://github.com/tokio-rs/prost) 
and [prost-reflect](https://github.com/andrewhickman/prost-reflect) generated code.

All validation rules are documented in the [proto file](prost-reflect-validate/proto/validate/validate.proto) 
or in the [protoc-gen-validate](https://github.com/bufbuild/protoc-gen-validate/blob/v1.1.0/README.md#constraint-rules) documentation.

**Proto definition**

```protobuf
syntax = "proto3";

package validate.example;

import "validate/validate.proto";

message ExampleMessage {
  string content = 1 [(validate.rules).string = {const: "Hello, world!"}];
}
```

**Validation**

It exposes a single extension trait `ValidatorExt` which can be used to validate protobuf reflect messages.

```rust
mod proto {
    use once_cell::sync::Lazy;
    use prost_reflect::DescriptorPool;

    static DESCRIPTOR_POOL: Lazy<DescriptorPool> = Lazy::new(|| DescriptorPool::decode(include_bytes!(concat!(env!("OUT_DIR"), "/file_descriptor_set.bin")).as_ref()).unwrap());
    include!(concat!(env!("OUT_DIR"), "/validate.example.rs"));
}

fn main() {
    use prost_reflect_validate::ValidatorExt;
    use crate::proto::ExampleMessage;

    match ExampleMessage::default().validate() {
        Ok(_) => println!("Validation passed"),
        Err(e) => eprintln!("Validation failed: {}", e),
    }
    let msg = ExampleMessage{content: "Hello, world!".to_string()};
    match msg.validate() {
        Ok(_) => println!("Validation passed"),
        Err(e) => eprintln!("Validation failed: {}", e),
    }
}
```

**Ouput**

```
Validation failed: validate.example.ExampleMessage.content: must be Hello, world!
Validation passed
```

## Minimum Supported Rust Version

Rust **1.64** or higher.

The minimum supported Rust version may be changed in the future, but it will be
done with a minor version bump.
