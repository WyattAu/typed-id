# Requirements — typed-id

Numbered, testable requirements. Every requirement maps to at least one named
test; every security-relevant test cites at least one requirement. Threat
IDs reference `THREAT-MODEL.md`.

Scope note: workspace publishing `typedids` (typed UUID newtypes via
`#[derive(TypedId)]`) and `typed-id-derive` (the proc-macro). Compile-time
ID-type separation, UUID v4/v7 generation with injectable `IdGenerator`,
nil handling, and string round-trips.

## Functional

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-TI-001 | `#[derive(TypedId)]` generates a newtype wrapper over `uuid::Uuid` with `new`, `parse`, `as_uuid`, and `is_nil` accessors | MUST |
| REQ-TI-002 | `parse` round-trips IDs produced from UUIDs (`to_string` → `parse`) without loss | MUST |
| REQ-TI-003 | `IdGenerator` is implemented by `SystemIdGenerator` (production RNG) and `FixedIdGenerator` (deterministic test seam); distinct instances produce distinct IDs where expected | MUST |
| REQ-TI-004 | Nil UUIDs are constructible and detectable via `is_nil` | SHOULD |
| REQ-TI-005 | Both workspace members enforce `#![forbid(unsafe_code)]` | MUST |

## Security

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-TI-100 | ID-type confusion is a compile error: distinct derive types are distinct types with no implicit cross-conversion (T1) | MUST |
| REQ-TI-101 | Parsing is delegated to `uuid::Uuid::parse_str` — malformed strings produce errors, never panics or silent truncation (T2) | MUST |
| REQ-TI-102 | Production generation uses the `uuid` crate's CSPRNG-backed v4 (or time-ordered v7); deterministic generators are opt-in and named for tests (T3, T4) | MUST |
| REQ-TI-103 | No `unsafe` anywhere in the workspace (T5) | MUST |

## Robustness

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-TI-200 | The crate is `no_std`-compatible (`core` + `alloc`) with `std` as the default feature | SHOULD |

## Traceability Matrix

| Requirement | Test (fn, file) | Property class |
|-------------|-----------------|----------------|
| REQ-TI-001 | `parse_works`, `nil_works`, `roundtrip_uuid` (`crates/typed-id/src/lib.rs` tests) | unit |
| REQ-TI-002 | `roundtrip_uuid`, `parse_works` | unit |
| REQ-TI-003 | `system_id_generator`, `different_instances_are_distinct` | unit |
| REQ-TI-004 | `nil_works` | unit |
| REQ-TI-005 | Crate attributes (`#![forbid(unsafe_code)]`) in both members | design |
| REQ-TI-100 | Type-system design (derive-generated distinct newtypes); compile-level guarantee | design |
| REQ-TI-101 | `parse_works` (error path) | unit |
| REQ-TI-102 | `system_id_generator`, `different_instances_are_distinct` | unit |
| REQ-TI-103 | Crate attributes | design |

## Test Count

- 5 `#[test]` functions across the workspace.
- All-features suite passes with 0 failures; no-default-features suite passes.
