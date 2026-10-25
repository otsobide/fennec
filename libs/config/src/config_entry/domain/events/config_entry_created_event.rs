//! Domain event raised when a new config entry is created.

use std::collections::BTreeMap;
use std::time::SystemTime;

use shared_domain_events::domain::domain_event::{DomainEvent, DomainEventBase};
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::config_entry::domain::value_objects::config_key::ConfigKey;
use crate::config_entry::domain::value_objects::config_value::ConfigValue;

/// Domain event raised when a new [`ConfigEntry`] is successfully persisted.
///
/// [`ConfigEntry`]: crate::config_entry::domain::entities::config_entry::ConfigEntry
pub struct ConfigEntryCreatedEvent {
    base: DomainEventBase,
    /// Key of the newly created entry.
    pub key: ConfigKey,
    /// Value stored under the key.
    pub value: ConfigValue,
}

impl ConfigEntryCreatedEvent {
    /// Canonical event name used to identify this event type on the bus.
    pub const EVENT_NAME: &'static str = "fennec.config.config_entry.created";

    /// Creates a new `ConfigEntryCreatedEvent` with auto-generated metadata.
    pub fn new(key: ConfigKey, value: ConfigValue) -> Self {
        Self {
            base: DomainEventBase::new(key.value().to_string()),
            key,
            value,
        }
    }
}

impl DomainEvent for ConfigEntryCreatedEvent {
    fn event_name(&self) -> &'static str {
        Self::EVENT_NAME
    }

    fn aggregate_id(&self) -> &str {
        &self.base.aggregate_id
    }

    fn event_id(&self) -> &str {
        &self.base.event_id
    }

    fn occurred_on(&self) -> SystemTime {
        self.base.occurred_on
    }

    fn to_primitives(&self) -> BTreeMap<String, PrimitiveValue> {
        BTreeMap::from([
            (
                "key".to_string(),
                PrimitiveValue::Text(self.key.value().to_string()),
            ),
            (
                "value".to_string(),
                PrimitiveValue::Text(self.value.value().to_string()),
            ),
        ])
    }
}
