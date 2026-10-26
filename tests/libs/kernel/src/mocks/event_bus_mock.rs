use std::collections::BTreeMap;
use std::sync::Mutex;

use shared_domain_events::domain::domain_event::DomainEvent;
use shared_domain_events::domain::event_bus::EventBus;
use shared_domain_events::domain::event_bus_error::EventBusError;
use shared_domain_events::domain::primitive_value::PrimitiveValue;

pub struct EventBusMock {
    should_fail: bool,
    published_event_names: Mutex<Vec<&'static str>>,
    published_primitives: Mutex<Vec<BTreeMap<String, PrimitiveValue>>>,
}

#[allow(dead_code)]
impl EventBusMock {
    pub fn new() -> Self {
        Self {
            should_fail: false,
            published_event_names: Mutex::new(vec![]),
            published_primitives: Mutex::new(vec![]),
        }
    }

    pub fn that_fails() -> Self {
        Self {
            should_fail: true,
            ..Self::new()
        }
    }

    pub fn published_event_names(&self) -> Vec<&'static str> {
        self.published_event_names.lock().unwrap().clone()
    }

    /// The payload of every published event, in the order of
    /// [`EventBusMock::published_event_names`].
    pub fn published_primitives(&self) -> Vec<BTreeMap<String, PrimitiveValue>> {
        self.published_primitives.lock().unwrap().clone()
    }
}

impl EventBus for EventBusMock {
    fn publish(&self, events: Vec<Box<dyn DomainEvent>>) -> Result<(), EventBusError> {
        if self.should_fail {
            return Err(EventBusError::DispatchError("mock error".to_string()));
        }
        let mut names = self.published_event_names.lock().unwrap();
        let mut primitives = self.published_primitives.lock().unwrap();
        for event in &events {
            names.push(event.event_name());
            primitives.push(event.to_primitives());
        }
        Ok(())
    }
}
