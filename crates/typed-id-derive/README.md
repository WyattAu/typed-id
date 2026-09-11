# typed-id-derive

[![docs.rs](https://docs.rs/typed-id-derive/badge.svg)](https://docs.rs/typed-id-derive)
[![crates.io](https://img.shields.io/crates/v/typed-id-derive.svg)](https://crates.io/crates/typed-id-derive)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](../../LICENSE)

Procedural macro backing [`typedids`](https://crates.io/crates/typedids):
`#[derive(TypedId)]` generates type-safe newtype wrappers around
`uuid::Uuid`, preventing confusion between different ID types at compile
time.

You normally depend on `typedids` and never on this crate directly:

```toml
[dependencies]
typedids = "0.1"
```

```rust,ignore
use typed_id_derive::TypedId;

#[derive(TypedId, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UserId(uuid::Uuid);
```

## License

MIT OR Apache-2.0
