use kernel::source::domain::events::create_source_updated_event::create_source_updated_event;
use shared_domain_events::domain::domain_event::DomainEvent;
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::src::source::domain::entities::mothers::source_mother::SourceMother;

#[test]
fn it_exposes_old_and_new_values_as_primitives() {
    let previous = SourceMother::random();
    let updated = SourceMother::random();

    let primitives = create_source_updated_event(&updated, &previous)
        .expect("event must be created")
        .to_primitives();

    assert_eq!(
        primitives.keys().collect::<Vec<_>>(),
        vec![
            "id",
            "new_description",
            "new_status",
            "old_description",
            "old_status",
            "updated_at"
        ]
    );
    assert_eq!(
        primitives["id"],
        PrimitiveValue::Text(updated.id().to_string())
    );
    assert_eq!(
        primitives["new_description"],
        PrimitiveValue::Text(updated.description().value().to_string())
    );
    assert_eq!(
        primitives["old_description"],
        PrimitiveValue::Text(previous.description().value().to_string())
    );
    assert_eq!(
        primitives["new_status"],
        PrimitiveValue::Text(updated.status().as_str().to_string())
    );
    assert_eq!(
        primitives["old_status"],
        PrimitiveValue::Text(previous.status().as_str().to_string())
    );
    assert_eq!(
        primitives["updated_at"],
        PrimitiveValue::Timestamp(updated.updated_at().value())
    );
}
