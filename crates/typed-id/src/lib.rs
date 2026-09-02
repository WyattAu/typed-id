#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Type-safe ID newtypes via derive macro.
//!
//! Provides a `#[derive(TypedId)]` macro that generates type-safe newtype
//! wrappers around `uuid::Uuid`, preventing confusion between different
//! ID types at compile time.

extern crate alloc;

pub use typed_id_derive::TypedId;
pub use uuid;

/// Trait for generating IDs.
///
/// Allows dependency injection of ID generation for testing.
pub trait IdGenerator {
    /// Generate a new ID.
    fn generate(&self) -> uuid::Uuid;
}

/// System ID generator using UUID v7 (time-ordered).
#[cfg(feature = "std")]
#[derive(Clone, Copy, Default, Debug)]
pub struct SystemIdGenerator;

#[cfg(feature = "std")]
impl IdGenerator for SystemIdGenerator {
    fn generate(&self) -> uuid::Uuid {
        uuid::Uuid::now_v7()
    }
}

/// Fixed ID generator for deterministic testing.
#[derive(Clone, Debug)]
pub struct FixedIdGenerator {
    id: uuid::Uuid,
}

impl FixedIdGenerator {
    /// Create a generator that always returns the given ID.
    pub fn new(id: uuid::Uuid) -> Self {
        Self { id }
    }
}

impl IdGenerator for FixedIdGenerator {
    fn generate(&self) -> uuid::Uuid {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[derive(TypedId, Clone, Copy, Debug, PartialEq, Eq, Hash)]
    struct TestId(uuid::Uuid);

    #[cfg(feature = "std")]
    #[test]
    fn roundtrip_uuid() {
        let id = TestId::new(uuid::Uuid::now_v7());
        let uuid = id.as_uuid();
        let back: uuid::Uuid = id.into();
        assert_eq!(uuid, back);
    }

    #[cfg(feature = "std")]
    #[test]
    fn parse_works() {
        let id = TestId::new(uuid::Uuid::now_v7());
        let parsed = TestId::parse(&id.to_string());
        assert_eq!(parsed, Some(id));
    }

    #[test]
    fn nil_works() {
        let id = TestId::nil();
        assert!(id.is_nil());
        assert_eq!(id.to_string(), "00000000-0000-0000-0000-000000000000");
    }
}
