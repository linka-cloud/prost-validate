[![crates.io](https://img.shields.io/crates/v/prost-validate.svg)](https://crates.io/crates/prost-validate/)
[![docs.rs](https://docs.rs/prost-validate/badge.svg)](https://docs.rs/prost-validate/)
[![deps.rs](https://deps.rs/crate/prost-validate/0.2.8/status.svg)](https://deps.rs/crate/prost-validate)
[![Continuous integration](https://github.com/linka-cloud/prost-validate/actions/workflows/ci_derive.yml/badge.svg)](https://github.com/linka-cloud/prost-validate/actions/workflows/ci_derive.yml)
![Apache 2.0](https://img.shields.io/badge/license-Apache2.0-blue.svg)

# `prost-validate`

A protobuf library extending [prost](https://github.com/tokio-rs/prost) with validation support.

This is a rust implementation of [protoc-gen-validate](https://github.com/bufbuild/protoc-gen-validate).

It uses the `prost` crate to generate the `derive` based validation code.

For a reflection based implementation see the [prost-reflect-validate](../prost-reflect-validate/README.md) crate.

## Usage

It must be used with [prost](https://github.com/tokio-rs/prost) generated code.

All validation rules are documented in the [proto file](../prost-validate-types/proto/validate/validate.proto) 
or in the [protoc-gen-validate](https://github.com/bufbuild/protoc-gen-validate/blob/v1.1.0/README.md#constraint-rules) documentation.

```bash
cargo add prost-validate --features derive
cargo add prost-validate-build --build
```

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

```rust no_run
fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_validate_build::Builder::new()
        .compile_protos(&["message.proto"], &["proto", "../prost-validate-types/proto"])?:
    Ok(())
}
```

**Validation**

### Include the generated code

```rust
include!(concat!(env!("OUT_DIR"), "/validate.example.rs"));
```

### Using the generated code

```rust
fn main() {
    use example_proto::ExampleMessage;
    use prost_validate::Validator;

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

Output:
> Validation failed: "validate.example.ExampleMessage.content": must be equal to "Hello, world!"
>
> Validation passed


