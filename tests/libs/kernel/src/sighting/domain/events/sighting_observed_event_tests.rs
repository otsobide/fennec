use std::time::SystemTime;

use kernel::sighting::domain::events::create_sighting_observed_event::create_sighting_observed_event;
use shared_domain_events::domain::domain_event::DomainEvent;
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::src::sighting::domain::entities::mothers::sighting_mother::SightingMother;

#[test]
fn it_exposes_the_observation_and_the_new_values_as_primitives() {
    let sighting = SightingMother::random();
    let observed_at = SystemTime::now();

    let primitives = create_sighting_observed_event(&sighting, observed_at)
        .expect("event must be created")
        .to_primitives();

    assert_eq!(
        primitives.keys().collect::<Vec<_>>(),
        vec![
            "id",
            "new_count",
            "new_last_seen",
            "new_updated_at",
            "observable_id",
            "observable_kind",
            "observed_at",
            "source_id"
        ]
    );
    assert_eq!(
        primitives["id"],
        PrimitiveValue::Text(sighting.id().to_string())
    );
    assert_eq!(
        primitives["observed_at"],
        PrimitiveValue::Timestamp(observed_at)
    );
    assert_eq!(
        primitives["new_count"],
        PrimitiveValue::UnsignedInteger(sighting.count().value())
    );
    assert_eq!(
        primitives["new_last_seen"],
        PrimitiveValue::Timestamp(sighting.last_seen().value())
    );
    assert_eq!(
        primitives["new_updated_at"],
        PrimitiveValue::Timestamp(sighting.updated_at().value())
    );
}
