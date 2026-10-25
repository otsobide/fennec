use kernel::url_source::domain::events::create_url_source_updated_event::create_url_source_updated_event;
use shared_domain_events::domain::domain_event::DomainEvent;
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::src::url_source::domain::entities::mothers::url_source_mother::UrlSourceMother;

#[test]
fn it_exposes_old_and_new_values_as_primitives() {
    let previous = UrlSourceMother::random();
    let updated = UrlSourceMother::random();

    let primitives = create_url_source_updated_event(&updated, &previous)
        .expect("event must be created")
        .to_primitives();

    assert_eq!(
        primitives.keys().collect::<Vec<_>>(),
        vec![
            "id",
            "new_format",
            "new_polling_interval",
            "new_url",
            "old_format",
            "old_polling_interval",
            "old_url",
            "updated_at"
        ]
    );
    assert_eq!(
        primitives["id"],
        PrimitiveValue::Text(updated.id().to_string())
    );
    assert_eq!(
        primitives["new_url"],
        PrimitiveValue::Text(updated.url().value().to_string())
    );
    assert_eq!(
        primitives["old_url"],
        PrimitiveValue::Text(previous.url().value().to_string())
    );
    assert_eq!(
        primitives["new_format"],
        PrimitiveValue::Text(updated.format().as_str().to_string())
    );
    assert_eq!(
        primitives["old_format"],
        PrimitiveValue::Text(previous.format().as_str().to_string())
    );
    assert_eq!(
        primitives["new_polling_interval"],
        PrimitiveValue::UnsignedInteger(u64::from(updated.polling_interval().seconds()))
    );
    assert_eq!(
        primitives["old_polling_interval"],
        PrimitiveValue::UnsignedInteger(u64::from(previous.polling_interval().seconds()))
    );
    assert_eq!(
        primitives["updated_at"],
        PrimitiveValue::Timestamp(updated.updated_at().value())
    );
}
