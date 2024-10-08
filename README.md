# Prost Validate

This is a Rust implementation of [protoc-gen-validate](https://github.com/bufbuild/protoc-gen-validate).

It must be used with [prost](https://github.com/tokio-rs/prost) generated code.

All validation rules are documented in the [proto file](../prost-validate-types/proto/validate/validate.proto)
or in the [protoc-gen-validate](https://github.com/bufbuild/protoc-gen-validate/blob/v1.1.0/README.md#constraint-rules) documentation.

It provides two implementations:
- A derive based implementation in the [prost-validate](prost-validate/README.md) crate.
- A reflection based implementation in the [prost-reflect-validate](prost-reflect-validate/README.md) crate.
