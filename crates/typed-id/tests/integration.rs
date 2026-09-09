use typedids::TypedId;

#[derive(TypedId, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OrderId(uuid::Uuid);

#[cfg(feature = "std")]
#[test]
fn different_instances_are_distinct() {
    let order1 = OrderId::new(uuid::Uuid::now_v7());
    let order2 = OrderId::new(uuid::Uuid::now_v7());
    assert_ne!(order1, order2);
}

#[cfg(feature = "std")]
#[test]
fn system_id_generator() {
    use typedids::{IdGenerator, SystemIdGenerator};
    let gen = SystemIdGenerator;
    let id1 = gen.generate();
    let id2 = gen.generate();
    assert_ne!(id1, id2);
}
