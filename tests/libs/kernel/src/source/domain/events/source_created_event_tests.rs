use kernel::source::domain::events::create_source_created_event::create_source_created_event;
use shared_domain_events::domain::domain_event::DomainEvent;
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::src::source::domain::entities::mothers::source_mother::SourceMother;

#[test]
fn it_exposes_the_full_state_as_primitives() {
    let source = SourceMother::random();

    let primitives = create_source_created_event(&source)
        .expect("event must be created")
        .to_primitives();

    assert_eq!(
        primitives.keys().collect::<Vec<_>>(),
        vec![
            "created_at",
            "description",
            "id",
            "source_type",
            "status",
            "updated_at"
        ]
    );
    assert_eq!(
        primitives["id"],
        PrimitiveValue::Text(source.id().to_string())
    );
    assert_eq!(
        primitives["source_type"],
        PrimitiveValue::Text(source.source_type().as_str().to_string())
    );
    assert_eq!(
        primitives["status"],
        PrimitiveValue::Text(source.status().as_str().to_string())
    );
    assert_eq!(
        primitives["description"],
        PrimitiveValue::Text(source.description().value().to_string())
    );
    assert_eq!(
        primitives["created_at"],
        PrimitiveValue::Timestamp(source.created_at().value())
    );
    assert_eq!(
        primitives["updated_at"],
        PrimitiveValue::Timestamp(source.updated_at().value())
    );
}
