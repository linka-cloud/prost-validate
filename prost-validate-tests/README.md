# `prost-validate-tests`

This crate contains the test suite for the `prost-validate` and `prost-reflect-validate` crates.

The test suite is adapted from the [protoc-gen-validate harness tests](https://github.com/bufbuild/protoc-gen-validate/blob/v1.1.0/tests/harness/executor/cases.go).

The `prost-validate` and `prost-reflect-validate` crates are tested against the same test suite to ensure that both implementations are compatible.

The `prost-validate` tests are behind the `derive` feature, and the `prost-reflect-validate` tests are behind the `reflect` feature.

The benchmark suite is also shared between the two implementations. It runs the test suite multiple times to measure the performance of the validation code.

## Usage


### Run the test suite for `prost-validate`

```bash
cargo test --features derive
```

### Run the test suite for `prost-reflect-validate`

```bash
cargo test --features reflect
```

### Run the test suite for both implementations

```bash
cargo test --all-features
```

### Run the benchmark suite

```bash
cargo bench --all-features
```
