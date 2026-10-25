use config::config_entry::domain::events::create_config_entry_updated_event::create_config_entry_updated_event;
use shared_domain_events::domain::domain_event::DomainEvent;
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::src::config_entry::domain::entities::mothers::config_entry_mother::ConfigEntryMother;

#[test]
fn it_exposes_old_and_new_values_as_primitives() {
    let previous = ConfigEntryMother::create("feeds.timeout", "30");
    let updated = ConfigEntryMother::create("feeds.timeout", "60");

    let primitives = create_config_entry_updated_event(&updated, &previous)
        .expect("event must be created")
        .to_primitives();

    assert_eq!(
        primitives.keys().collect::<Vec<_>>(),
        vec!["key", "new_value", "old_value"]
    );
    assert_eq!(
        primitives["key"],
        PrimitiveValue::Text("feeds.timeout".to_string())
    );
    assert_eq!(
        primitives["new_value"],
        PrimitiveValue::Text("60".to_string())
    );
    assert_eq!(
        primitives["old_value"],
        PrimitiveValue::Text("30".to_string())
    );
}
