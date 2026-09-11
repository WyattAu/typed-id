# Threat Model — typed-id

Status: **v1.0** · Method: STRIDE over the public API surface
(`TypedId` derive, `IdGenerator`, `SystemIdGenerator`, `FixedIdGenerator`,
typed newtypes such as `UserId`).

Trust boundaries: (1) serialized ID strings arriving from clients
(paths, JSON bodies, query strings), (2) the UUID parsing layer, (3)
ID *generation* (system RNG vs injected generators in tests).

## Assets

| ID | Asset | Example |
|----|-------|---------|
| A1 | Type-level ID separation | A `UserId` accepted where an `OrderId` is required (ID-confusion privilege escalation) |
| A2 | Parse correctness | Truncated or hostile strings parsed into "valid" IDs |
| A3 | Uniqueness of generated IDs | Predictable or repeated IDs in production (or in tests, false confidence) |
| A4 | Memory safety | `unsafe` in the newtype/derive path |

## STRIDE Analysis

| # | Threat | Category | Surface | Mitigation | Verifying test |
|---|--------|----------|---------|------------|----------------|
| T1 | ID-type confusion (UserId passed as OrderId) | Spoofing/Elevation | all newtypes | Each ID is a distinct newtype; no `From<Uuid>`-style cross-construction — cross-type assignment is a compile error by construction | Design (type system); `roundtrip_uuid`, `parse_works` |
| T2 | Malformed/hostile ID strings accepted | Tampering | `parse` | Parsing delegates to `uuid::Uuid::parse_str` with typed errors; nil and malformed inputs rejected or explicitly represented (`is_nil`) | `parse_works`, `nil_works` |
| T3 | ID forgery by prediction (v4 randomness or v7 ordering) | Spoofing | `SystemIdGenerator` | Generation delegates to `uuid` v4 (CSPRNG) / v7 (time-ordered); the crate adds no deterministic fallback in production — `FixedIdGenerator` is an explicit, named test seam | `system_id_generator`, `different_instances_are_distinct` |
| T4 | Generator injection weakening production randomness | Elevation | `IdGenerator` | Injection is opt-in via the trait; production default is `SystemIdGenerator`, and `FixedIdGenerator` produces a *fixed* value that is obvious in review | `different_instances_are_distinct`, `system_id_generator` |
| T5 | Memory unsafety in derive/newtype code | Tampering | crate-wide | `#![forbid(unsafe_code)]` at the crate root (both members) — no unsafe exists to exploit | Cargo.toml/crate attrs; `cargo clippy` gate |

## OPEN RISKS (missing mitigations — not fabricated)

- **OPEN-1 — no built-in MAC/signature for IDs exposed to clients.**
  Typed IDs prevent *confusion*, not *forgery by enumeration*: callers
  exposing sequential v7 IDs publicly should add their own opacity or
  authorization layer.
- **OPEN-2 — serde representation is the UUID string form.** Callers
  needing opaque external representations must wrap at their API edge.

## Out of Scope

- Authorization (whether a caller may *use* a given ID).
- Database column security; storage-level access control.
- Alternative ID schemes (ULID, KSUID) — documented tradeoff in crate
  docs.

## Residual Risks

- UUIDv7 IDs leak creation timestamps by design (sortable IDs); callers
  treating creation time as sensitive must not expose raw v7 values.
