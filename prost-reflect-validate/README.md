# `prost-reflect-validate`

A protobuf library extending [prost](https://github.com/tokio-rs/prost)
and [prost-reflect](https://github.com/andrewhickman/prost-reflect) with validation support.

This is a rust implementation of [protoc-gen-validate](https://github.com/bufbuild/protoc-gen-validate).

It uses the `prost-reflect` crate to implement validation through reflection.

For a *derive* based implementation see the [prost-validate](../prost-validate/README.md) crate.

## Usage

It must be used with [prost](https://github.com/tokio-rs/prost) 
and [prost-reflect](https://github.com/andrewhickman/prost-reflect) generated code.

All validation rules are documented in the [proto file](../prost-validate-types/proto/validate/validate.proto) 
or in the [protoc-gen-validate](https://github.com/bufbuild/protoc-gen-validate/blob/v1.1.0/README.md#constraint-rules) documentation.

**Proto definition**

`proto/message.proto`:

```protobuf
syntax = "proto3";

package validate.example;

import "validate/validate.proto";

message ExampleMessage {
  string content = 1 [(validate.rules).string = {const: "Hello, world!"}];
}
```

**Build script**

`build.rs`:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_reflect_build::Builder::new()
        .descriptor_pool("DESCRIPTOR_POOL")
        .compile_protos(&["message.proto"], &["proto", "../prost-validate-types/proto"])
}
```

**Validation**

It exposes a single extension trait `ValidatorExt` which can be used to validate protobuf reflect messages.

`src/main.rs`:

```rust
fn main() {
    use example_proto::ExampleMessage;
    use prost_reflect_validate::ValidatorExt;

    match ExampleMessage::default().validate() {
        Ok(_) => println!("Validation passed"),
        Err(e) => eprintln!("Validation failed: {}", e),
    }
    let msg = ExampleMessage {
        content: "Hello, world!".to_string(),
    };
    match msg.validate() {
        Ok(_) => println!("Validation passed"),
        Err(e) => eprintln!("Validation failed: {}", e),
    }
}
```

**Output**

> Validation failed: "validate.example.ExampleMessage.content": must be equal to "Hello, world!"
>
> Validation passed


## Minimum Supported Rust Version

Rust **1.64** or higher.

The minimum supported Rust version may be changed in the future, but it will be
done with a minor version bump.
