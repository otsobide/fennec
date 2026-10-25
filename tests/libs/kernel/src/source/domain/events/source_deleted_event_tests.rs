use kernel::source::domain::events::create_source_deleted_event::create_source_deleted_event;
use shared_domain_events::domain::domain_event::DomainEvent;
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::src::source::domain::entities::mothers::source_mother::SourceMother;

#[test]
fn it_exposes_the_identity_as_primitives() {
    let source = SourceMother::random();

    let primitives = create_source_deleted_event(&source)
        .expect("event must be created")
        .to_primitives();

    assert_eq!(primitives.keys().collect::<Vec<_>>(), vec!["id"]);
    assert_eq!(
        primitives["id"],
        PrimitiveValue::Text(source.id().to_string())
    );
}
