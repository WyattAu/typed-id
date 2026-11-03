# typedids
[![Estate integration](https://github.com/WyattAu/estate-integration/actions/workflows/ci.yml/badge.svg)](https://github.com/WyattAu/estate-integration/actions/workflows/ci.yml)

[![docs.rs](https://docs.rs/typedids/badge.svg)](https://docs.rs/typedids)
[![crates.io](https://img.shields.io/crates/v/typedids.svg)](https://crates.io/crates/typedids)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](../../LICENSE)

Type-safe ID newtypes via derive macro. A `#[derive(TypedId)]` macro that
generates type-safe newtype wrappers around `uuid::Uuid`, preventing
confusion between different ID types at compile time.

## Publishing status

**Not yet on crates.io.** The planned package name is
[`typedids`](https://crates.io/crates/typedids) (the `typed-id` name is
taken by an unrelated crate; `typed-id-new` was a placeholder that was
published as 0.0.0 and should be considered vestigial). The
[`typed-id-derive`](https://crates.io/crates/typed-id-derive) proc-macro
crate **is published** (0.1.1) and is consumed directly by the estate's
integration suite. Repo directory (`typed-id`) is unchanged.

## Usage

```toml
[dependencies]
typedids = "0.1"
```

```rust,ignore
use typedids::TypedId;

#[derive(TypedId, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UserId(uuid::Uuid);

let id = UserId::new(uuid::Uuid::now_v7());
assert_eq!(id.as_uuid(), id.into());
```

Generated API per ID type: `new`, `as_uuid`, `parse`, `nil`, `is_nil`,
`Display`, `From<Uuid>`, `Into<Uuid>`. `SystemIdGenerator` /
`FixedIdGenerator` cover the `IdGenerator` trait for production vs tests.

## Why not ULID?

Typed UUIDv7 newtypes with serde/derive ergonomics vs ULID's
Crockford-base32 sortable IDs; different tradeoffs, ours integrates with
existing UUID columns.
