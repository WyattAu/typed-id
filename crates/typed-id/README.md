# typedids

Type-safe ID newtypes via derive macro. A `#[derive(TypedId)]` macro that
generates type-safe newtype wrappers around `uuid::Uuid`, preventing
confusion between different ID types at compile time.

Published on crates.io as
[`typedids`](https://crates.io/crates/typedids) (`typed-id` is taken by an
unrelated crate; `typed-id-new` was a placeholder name). Repo directory
(`typed-id`) is unchanged.

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
Crockford-base32 sortable IDs — different tradeoffs, ours integrates with
existing UUID columns.
