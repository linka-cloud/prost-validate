[![crates.io](https://img.shields.io/crates/v/prost-validate-build.svg)](https://crates.io/crates/prost-validate-build/)
[![docs.rs](https://docs.rs/prost-validate-build/badge.svg)](https://docs.rs/prost-validate-build/)
[![deps.rs](https://deps.rs/crate/prost-validate-build/0.2.1/status.svg)](https://deps.rs/crate/prost-validate-build)
![MSRV](https://img.shields.io/badge/rustc-1.74+-blue.svg)
[![Continuous integration](https://github.com/linka-cloud/prost-validate/actions/workflows/ci_derive.yml/badge.svg)](https://github.com/linka-cloud/prost-validate/actions/workflows/ci_derive.yml)
![Apache 2.0](https://img.shields.io/badge/license-Apache2.0-blue.svg)

# `prost-validate-build`

A protobuf library extending [prost](https://github.com/tokio-rs/prost) with validation support.

`prost-validate-build` is a crate that can be used to generate protobuf message validation from
[protoc-gen-validate](https://github.com/bufbuild/protoc-gen-validate) annotations.

This crate is intended to be used in a `build.rs` script to generate the validation code for the messages.

It depends on the `prost-validate` crate's `derive` feature to generate the validation code.

## Usage

It can be used to compile the `.proto` files into Rust using [`prost-build`](https://docs.rs/prost-build)
or to simply generate the `prost-build` configuration.

All validation rules are documented in the [proto file](../prost-validate-types/proto/validate/validate.proto)
or in the [protoc-gen-validate](https://github.com/bufbuild/protoc-gen-validate/blob/v1.1.0/README.md#constraint-rules) documentation.

```bash
cargo add prost-validate --features derive
cargo add prost-validate-build --build
```

### Example Protobuf definition

`proto/message.proto`:

```proto
syntax = "proto3";

package validate.example;

import "validate/validate.proto";

message ExampleMessage {
  string content = 1 [(validate.rules).string = {const: "Hello, world!"}];
}
```

### Generating Rust code with validation


`build.rs`:

```rust no_run
fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_validate_build::Builder::new()
        .compile_protos(&["message.proto"], &["proto", "../prost-validate-types/proto"])?;
    Ok(())
}
```

### Generating `prost-build` configuration for usage with other generators

#### Example for `prost-reflect-build`

`build.rs`:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let files = &["message.proto"];
    let includes = &["proto", "../prost-validate-types/proto"];

    let mut config = prost_build::Config::new();

    prost_validate_build::Builder::new().configure(&mut config, files, includes)?;

    prost_reflect_build::Builder::new()
        .descriptor_pool("DESCRIPTOR_POOL")
        .compile_protos_with_config(config, files, includes)?;

    Ok(())
}
```

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


