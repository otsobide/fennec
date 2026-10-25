use kernel::url_source::domain::events::create_url_source_deleted_event::create_url_source_deleted_event;
use shared_domain_events::domain::domain_event::DomainEvent;
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::src::url_source::domain::entities::mothers::url_source_mother::UrlSourceMother;

#[test]
fn it_exposes_the_identity_as_primitives() {
    let url_source = UrlSourceMother::random();

    let primitives = create_url_source_deleted_event(&url_source)
        .expect("event must be created")
        .to_primitives();

    assert_eq!(primitives.keys().collect::<Vec<_>>(), vec!["id"]);
    assert_eq!(
        primitives["id"],
        PrimitiveValue::Text(url_source.id().to_string())
    );
}
