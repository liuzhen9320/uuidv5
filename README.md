# `uuidv5`

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)
[![Latest version](https://img.shields.io/crates/v/uuidv5.svg)](https://crates.io/crates/uuidv5)
[![Documentation](https://docs.rs/uuidv5/badge.svg)](https://docs.rs/uuidv5)
![License](https://img.shields.io/crates/l/uuidv5.svg)

A `#![no_std]`, zero-allocation UUID v5 (SHA-1 based) implementation for Rust.

```text
67555272-4d7a-5631-9cdd-87b678306a62
```

UUID v5 generates a deterministic 128-bit value from a namespace UUID and a
name, using SHA-1 hashing. Version 5 UUIDs are ideal when you need the same
input to always produce the same UUID, such as content-addressable identifiers
in distributed systems, database keys derived from natural data, or stable IDs
across service boundaries.

## Getting started

Add `uuidv5` to your `Cargo.toml`:

```toml
[dependencies]
uuidv5 = "0.1"
```

Generate a UUID v5:

```rust
use uuidv5::{Uuid, new};

let id = new(Uuid::NAMESPACE_DNS, b"example.com");
```

Or use the convenience macro:

```rust
use uuidv5::uuidv5;

let id = uuidv5!(uuidv5::Uuid::NAMESPACE_DNS, b"example.com");
```

## Namespace constants

Four predefined namespace UUIDs are provided:

| Constant               | Value                                  |
| ---------------------- | -------------------------------------- |
| `Uuid::NAMESPACE_DNS`  | `6ba7b810-9dad-11d1-80b4-00c04fd430c8` |
| `Uuid::NAMESPACE_URL`  | `6ba7b811-9dad-11d1-80b4-00c04fd430c8` |
| `Uuid::NAMESPACE_OID`  | `6ba7b812-9dad-11d1-80b4-00c04fd430c8` |
| `Uuid::NAMESPACE_X500` | `6ba7b814-9dad-11d1-80b4-00c04fd430c8` |

## Parsing

Parse a UUID v5 from its standard string form:

```rust
use uuidv5::Uuid;

let u = Uuid::try_from("2ed6657d-e927-568b-95e1-2665a8aea6a2")?; // DNS + www.example.com
```

Parsing validates length, character set, hyphen placement, version (must be 5),
and variant (must be RFC 4122). On failure, a `ParseError` is returned:

```rust
use uuidv5::ParseError;

assert_eq!(
    Uuid::try_from("not-a-uuid"),
    Err(ParseError::InvalidLength),
);
```

## `#![no_std]` support

This crate targets embedded and bare-metal environments. The only dependency
is `sha1` (with default features disabled). No heap allocations, no `std`, no
`alloc`.

## API overview

| Item                         | Description                     |
| ---------------------------- | ------------------------------- |
| `new(namespace, name)`       | Generate a UUID v5              |
| `uuidv5!(namespace, name)`   | Macro form of `new`             |
| `Uuid::NAMESPACE_DNS`        | DNS namespace                   |
| `Uuid::NAMESPACE_URL`        | URL namespace                   |
| `Uuid::NAMESPACE_OID`        | OID namespace                   |
| `Uuid::NAMESPACE_X500`       | X.500 namespace                 |
| `Uuid::from_bytes([u8; 16])` | Construct from raw bytes        |
| `Uuid::as_bytes()`           | Borrow inner byte array         |
| `Uuid::try_from(&str)`       | Parse from string               |
| `Uuid::to_string()`          | Format as standard string       |
| `ParseError`                 | Error type for parsing failures |

## Minimum supported Rust version

The MSRV is 1.85.0.

## References

- [Wikipedia: Universally Unique Identifier](https://en.wikipedia.org/wiki/Universally_unique_identifier)
- [RFC 9562: Universally Unique IDentifiers (UUID)](https://www.ietf.org/rfc/rfc9562.html)
- [Section 5.5: UUID Version 5 (SHA-1)](https://www.ietf.org/rfc/rfc9562.html#name-uuid-version-5)

## License

Licensed under the MIT license.
