use kernel::sighting::domain::events::create_sighting_created_event::create_sighting_created_event;
use shared_domain_events::domain::domain_event::DomainEvent;
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::src::sighting::domain::entities::mothers::sighting_mother::SightingMother;

#[test]
fn it_exposes_the_full_state_as_primitives() {
    let sighting = SightingMother::random();

    let primitives = create_sighting_created_event(&sighting)
        .expect("event must be created")
        .to_primitives();

    assert_eq!(
        primitives.keys().collect::<Vec<_>>(),
        vec![
            "count",
            "created_at",
            "first_seen",
            "id",
            "ioc_id",
            "last_seen",
            "source_id",
            "updated_at"
        ]
    );
    assert_eq!(
        primitives["id"],
        PrimitiveValue::Text(sighting.id().to_string())
    );
    assert_eq!(
        primitives["ioc_id"],
        PrimitiveValue::Text(sighting.ioc_id().to_string())
    );
    assert_eq!(
        primitives["source_id"],
        PrimitiveValue::Text(sighting.source_id().to_string())
    );
    assert_eq!(
        primitives["first_seen"],
        PrimitiveValue::Timestamp(sighting.first_seen().value())
    );
    assert_eq!(
        primitives["last_seen"],
        PrimitiveValue::Timestamp(sighting.last_seen().value())
    );
    assert_eq!(
        primitives["count"],
        PrimitiveValue::UnsignedInteger(sighting.count().value())
    );
    assert_eq!(
        primitives["created_at"],
        PrimitiveValue::Timestamp(sighting.created_at().value())
    );
    assert_eq!(
        primitives["updated_at"],
        PrimitiveValue::Timestamp(sighting.updated_at().value())
    );
}
