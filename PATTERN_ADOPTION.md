# PATTERN_ADOPTION: typedids

Relationship to the WyattAu pattern canon. Statuses follow the closed
vocabulary: adopted, partial, planned, refused, n/a. Evidence paths are
relative to this repo.

| Pattern | Status | Evidence |
|---|---|---|
| P60 | partial | Default generator emits time-ordered UUIDv7: crates/typed-id/src/lib.rs:34-44 (SystemIdGenerator calls now_v7), crates/typed-id/src/lib.rs:15-19 (sortable-ID tradeoff note). Time-ordering is a default, not an enforced invariant: the derive wraps any Uuid, and hosts can suppress the generated constructor |
| T48 | n/a | Planning guessed T48 as "newtype IDs"; the catalog defines T48 as grammar-constrained decoding, unrelated to this crate, and no canon ID covers opaque newtype identifiers |
