# uuidv5

[![CI](https://github.com/liuzhen932/uuidv5/actions/workflows/ci.yml/badge.svg)](https://github.com/liuzhen932/uuidv5/actions/workflows/ci.yml)

A `#![no_std]` UUID v5 (SHA-1 based) implementation for Rust.

Zero allocations. Minimal dependencies (only `sha1`).

## Usage

```rust
use uuidv5::{Uuid, new};

let id = new(Uuid::NAMESPACE_DNS, b"example.com");
println!("{}", id);
```

## API

### `uuidv5::new(namespace: Uuid, name: &[u8]) -> Uuid`

Generate a UUID v5 from a namespace UUID and a name.

### `Uuid`

A 16-byte UUID with the following traits implemented:

- `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`
- `core::fmt::Display` — outputs standard `xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx` format
- `From<[u8; 16]>` — construct from raw bytes
- `TryFrom<&str>` — parse from standard string format

### Namespace Constants

- `Uuid::NAMESPACE_DNS` — `6ba7b810-9dad-11d1-80b4-00c04fd430c8`
- `Uuid::NAMESPACE_URL` — `6ba7b811-9dad-11d1-80b4-00c04fd430c8`
- `Uuid::NAMESPACE_OID` — `6ba7b812-9dad-11d1-80b4-00c04fd430c8`
- `Uuid::NAMESPACE_X500` — `6ba7b814-9dad-11d1-80b4-00c04fd430c8`

### Parsing

```rust
use uuidv5::Uuid;

let u = Uuid::try_from("2ed6657d-e927-568b-95e1-2665a8aea6a2")?;
```

Parsing validates length (36), hex characters, hyphens, version (must be 5), and variant (must be `10xx`).

### Error Type

```rust
pub enum ParseError {
    InvalidLength,
    InvalidCharacter(char, usize),
    InvalidVersion(u8),
    InvalidVariant(u8),
}
```

## License

MIT
