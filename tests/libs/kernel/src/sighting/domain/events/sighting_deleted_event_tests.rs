use kernel::sighting::domain::events::create_sighting_deleted_event::create_sighting_deleted_event;
use shared_domain_events::domain::domain_event::DomainEvent;
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::src::sighting::domain::entities::mothers::sighting_mother::SightingMother;

#[test]
fn it_exposes_the_identity_and_the_pair_as_primitives() {
    let sighting = SightingMother::random();

    let primitives = create_sighting_deleted_event(&sighting)
        .expect("event must be created")
        .to_primitives();

    assert_eq!(
        primitives.keys().collect::<Vec<_>>(),
        vec!["id", "observable_id", "observable_kind", "source_id"]
    );
    assert_eq!(
        primitives["id"],
        PrimitiveValue::Text(sighting.id().to_string())
    );
    assert_eq!(
        primitives["observable_kind"],
        PrimitiveValue::Text(sighting.observable_kind().to_string())
    );
    assert_eq!(
        primitives["observable_id"],
        PrimitiveValue::Text(sighting.observable_id().to_string())
    );
    assert_eq!(
        primitives["source_id"],
        PrimitiveValue::Text(sighting.source_id().to_string())
    );
}
