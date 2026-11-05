//! `#[id(...)]` suppression knobs: hosts whose macro already emits a
//! random-ID `new()` or a by-reference `as_uuid` keep their own versions
//! without conflicting with the generated impls.

use typedids::TypedId;
use uuid::Uuid;

#[derive(TypedId, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[id(new = false, as_uuid = false)]
struct HostManagedId(Uuid);

impl HostManagedId {
    /// Host policy: random v7 (the engine macro owns ID-version policy).
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }

    /// Host policy: by-reference accessor.
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

#[test]
fn suppressed_items_do_not_conflict_and_generated_items_remain() {
    let id = HostManagedId::generate();
    // Host's own by-reference accessor works.
    assert_eq!(*id.as_uuid(), id.0);
    // Generated parse/nil/is_nil/Display/From are still present.
    let parsed = HostManagedId::parse(&id.0.hyphenated().to_string()).expect("parse");
    assert_eq!(parsed, id);
    assert!(!id.is_nil());
    assert_eq!(HostManagedId::nil().0, Uuid::nil());
    let round: Uuid = id.into();
    let back: HostManagedId = round.into();
    assert_eq!(back, id);
}

#[test]
fn default_generate_still_exists_without_knobs() {
    #[derive(TypedId, Clone, Copy, Debug, PartialEq, Eq, Hash)]
    struct PlainId(Uuid);

    let id = PlainId::new(Uuid::now_v7());
    assert_eq!(id.as_uuid(), id.0);
}
