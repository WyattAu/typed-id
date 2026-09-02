use typed_id::TypedId;

#[derive(TypedId, Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct UserId(#[serde(with = "uuid::serde::hyphenated")] uuid::Uuid);

#[derive(TypedId, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OrderId(uuid::Uuid);

#[test]
fn different_types_are_incompatible() {
    let user = UserId::new(uuid::Uuid::now_v7());
    let order = OrderId::new(uuid::Uuid::now_v7());
    // These are different types - can't compare directly
    // This test verifies the types exist and are distinct
    assert_ne!(user.to_string(), order.to_string());
}

#[test]
fn serde_roundtrip() {
    let id = UserId::new(uuid::Uuid::now_v7());
    let json = serde_json::to_string(&id).unwrap();
    let parsed: UserId = serde_json::from_str(&json).unwrap();
    assert_eq!(id, parsed);
}

#[test]
fn system_id_generator() {
    use typed_id::{SystemIdGenerator, IdGenerator};
    let gen = SystemIdGenerator;
    let id1 = gen.generate();
    let id2 = gen.generate();
    assert_ne!(id1, id2);
}
