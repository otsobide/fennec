use std::sync::Mutex;

use shared_domain_events::domain::domain_event::DomainEvent;
use shared_domain_events::domain::event_bus::EventBus;
use shared_domain_events::domain::event_bus_error::EventBusError;

#[derive(Default)]
pub struct RecordingEventBus {
    published: Mutex<Vec<&'static str>>,
}

#[allow(dead_code)]
impl RecordingEventBus {
    pub fn published_event_names(&self) -> Vec<&'static str> {
        self.published.lock().unwrap().clone()
    }
}

impl EventBus for RecordingEventBus {
    fn publish(&self, events: Vec<Box<dyn DomainEvent>>) -> Result<(), EventBusError> {
        let mut published = self.published.lock().unwrap();
        published.extend(events.iter().map(|event| event.event_name()));
        Ok(())
    }
}
