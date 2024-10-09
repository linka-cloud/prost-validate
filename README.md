![MSRV](https://img.shields.io/badge/rustc-1.74+-blue.svg)
[![Continuous integration](https://github.com/linka-cloud/prost-validate/actions/workflows/ci_derive.yml/badge.svg)](https://github.com/linka-cloud/prost-validate/actions/workflows/ci_derive.yml)
![Apache 2.0](https://img.shields.io/badge/license-Apache2.0-blue.svg)

# Prost Validate

This is a Rust implementation of [protoc-gen-validate](https://github.com/bufbuild/protoc-gen-validate).

It must be used with [prost](https://github.com/tokio-rs/prost) generated code.

All validation rules are documented in the [proto file](../prost-validate-types/proto/validate/validate.proto)
or in the [protoc-gen-validate](https://github.com/bufbuild/protoc-gen-validate/blob/v1.1.0/README.md#constraint-rules)
documentation.

It provides two implementations:

- A derive based implementation in the [prost-validate](prost-validate/README.md) crate.
- A reflection based implementation in the [prost-reflect-validate](prost-reflect-validate/README.md) crate.

The [test suite](prost-validate-tests) adapted from
the [protoc-gen-validate harness tests](https://github.com/bufbuild/protoc-gen-validate/blob/v1.1.0/tests/harness/executor/cases.go)
is shared between the two implementations.

Here are the benchmarks for the tests suite of the two implementations:

`prost-reflect-validate`:

```
harness reflect         time:   [14.849 ms 15.128 ms 15.459 ms]
```

`prost-validate`:

```
harness derive          time:   [2.5635 ms 2.5780 ms 2.5967 ms]
```

### Constraint Rule Comparison

#### Global

| Constraint Rule | Derive | Reflect |
|-----------------|:------:|:-------:|
| disabled        |   ✅    |    ✅    |

#### Numerics

| Constraint Rule | Derive | Reflect |
|-----------------|:------:|:-------:|
| const           |   ✅    |    ✅    |
| lt/lte/gt/gte   |   ✅    |    ✅    |
| in/not_in       |   ✅    |    ✅    |

#### Bools

| Constraint Rule | Derive | Reflect |
|-----------------|:------:|:-------:|
| const           |   ✅    |    ✅    |

#### Strings

| Constraint Rule        | Derive | Reflect |
|------------------------|:------:|:-------:|
| const                  |   ✅    |    ✅    |
| len/min\_len/max_len   |   ✅    |    ✅    |
| min\_bytes/max\_bytes  |   ✅    |    ✅    |
| pattern                |   ✅    |    ✅    |
| prefix/suffix/contains |   ✅    |    ✅    |
| contains/not_contains  |   ✅    |    ✅    |
| in/not_in              |   ✅    |    ✅    |
| email                  |   ✅    |    ✅    |
| hostname               |   ✅    |    ✅    |
| address                |   ✅    |    ✅    |
| ip                     |   ✅    |    ✅    |
| ipv4                   |   ✅    |    ✅    |
| ipv6                   |   ✅    |    ✅    |
| uri                    |   ✅    |    ✅    |
| uri_ref                |   ✅    |    ✅    |
| uuid                   |   ✅    |    ✅    |
| well_known_regex       |   ✅    |    ✅    |

#### Bytes

| Constraint Rule        | Derive | Reflect |
|------------------------|:------:|:-------:|
| const                  |   ✅    |    ✅    |
| len/min\_len/max_len   |   ✅    |    ✅    |
| pattern                |   ✅    |    ✅    |
| prefix/suffix/contains |   ✅    |    ✅    |
| in/not_in              |   ✅    |    ✅    |
| ip                     |   ✅    |    ✅    |
| ipv4                   |   ✅    |    ✅    |
| ipv6                   |   ✅    |    ✅    |

#### Enums

| Constraint Rule | Derive | Reflect |
|-----------------|:------:|:-------:|
| const           |   ✅    |    ✅    |
| defined_only    |   ✅    |    ✅    |
| in/not_in       |   ✅    |    ✅    |

#### Messages

| Constraint Rule | Derive | Reflect |
|-----------------|:------:|:-------:|
| skip            |   ✅    |    ✅    |
| required        |   ✅    |    ✅    |

#### Repeated

| Constraint Rule      | Derive | Reflect |
|----------------------|:------:|:-------:|
| min\_items/max_items |   ✅    |    ✅    |
| unique               |   ✅    |    ✅    |
| items                |   ✅    |    ✅    |

#### Maps

| Constraint Rule      | Derive | Reflect |
|----------------------|:------:|:-------:|
| min\_pairs/max_pairs |   ✅    |    ✅    |
| no_sparse            |   ❓    |    ❓    |
| keys                 |   ✅    |    ✅    |
| values               |   ✅    |    ✅    |

#### OneOf

| Constraint Rule | Derive | Reflect |
|-----------------|:------:|:-------:|
| required        |   ✅    |    ✅    |

#### WKT Scalar Value Wrappers

| Constraint Rule    | Derive | Reflect |
|--------------------|:------:|:-------:|
| wrapper validation |   ✅    |    ✅    |

#### WKT Any

| Constraint Rule | Derive | Reflect |
|-----------------|:------:|:-------:|
| required        |   ✅    |    ✅    |
| in/not_in       |   ✅    |    ✅    |

#### WKT Duration

| Constraint Rule | Derive | Reflect |
|-----------------|:------:|:-------:|
| required        |   ✅    |    ✅    |
| const           |   ✅    |    ✅    |
| lt/lte/gt/gte   |   ✅    |    ✅    |
| in/not_in       |   ✅    |    ✅    |

#### WKT Timestamp

| Constraint Rule | Derive | Reflect |
|-----------------|:------:|:-------:|
| required        |   ✅    |    ✅    |
| const           |   ✅    |    ✅    |
| lt/lte/gt/gte   |   ✅    |    ✅    |
| lt_now/gt_now   |   ✅    |    ✅    |
| within          |   ✅    |    ✅    |
