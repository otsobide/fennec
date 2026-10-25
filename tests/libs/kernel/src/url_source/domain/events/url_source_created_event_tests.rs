use kernel::url_source::domain::events::create_url_source_created_event::create_url_source_created_event;
use shared_domain_events::domain::domain_event::DomainEvent;
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::src::url_source::domain::entities::mothers::url_source_mother::UrlSourceMother;

#[test]
fn it_exposes_the_full_state_as_primitives() {
    let url_source = UrlSourceMother::random();

    let primitives = create_url_source_created_event(&url_source)
        .expect("event must be created")
        .to_primitives();

    assert_eq!(
        primitives.keys().collect::<Vec<_>>(),
        vec![
            "created_at",
            "format",
            "id",
            "polling_interval",
            "updated_at",
            "url"
        ]
    );
    assert_eq!(
        primitives["id"],
        PrimitiveValue::Text(url_source.id().to_string())
    );
    assert_eq!(
        primitives["url"],
        PrimitiveValue::Text(url_source.url().value().to_string())
    );
    assert_eq!(
        primitives["format"],
        PrimitiveValue::Text(url_source.format().as_str().to_string())
    );
    assert_eq!(
        primitives["polling_interval"],
        PrimitiveValue::UnsignedInteger(u64::from(url_source.polling_interval().seconds()))
    );
    assert_eq!(
        primitives["created_at"],
        PrimitiveValue::Timestamp(url_source.created_at().value())
    );
    assert_eq!(
        primitives["updated_at"],
        PrimitiveValue::Timestamp(url_source.updated_at().value())
    );
}
