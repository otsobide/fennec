use config::config_entry::domain::events::create_config_entry_deleted_event::create_config_entry_deleted_event;
use shared_domain_events::domain::domain_event::DomainEvent;
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::src::config_entry::domain::entities::mothers::config_entry_mother::ConfigEntryMother;

#[test]
fn it_exposes_the_identity_as_primitives() {
    let entry = ConfigEntryMother::random();

    let primitives = create_config_entry_deleted_event(&entry)
        .expect("event must be created")
        .to_primitives();

    assert_eq!(primitives.keys().collect::<Vec<_>>(), vec!["key"]);
    assert_eq!(
        primitives["key"],
        PrimitiveValue::Text(entry.key().value().to_string())
    );
}
