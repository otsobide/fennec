//! The event bus of commands that persist in PostgreSQL.

use shared_domain_events::domain::domain_event::DomainEvent;
use shared_domain_events::domain::event_bus::EventBus;
use shared_domain_events::domain::event_bus_error::EventBusError;

use crate::infrastructure::postgres::pg_unit_of_work::PgUnitOfWork;

/// An [`EventBus`] that hands published events to the current
/// [`PgUnitOfWork`] instead of delivering them.
///
/// The unit of work appends them to `event_store.domain_event` in the same
/// transaction as the state they describe, and only then delivers them to
/// in-process subscribers. Publishing outside a unit of work is a wiring
/// error and fails, so a missing transactional decorator cannot go unnoticed.
#[derive(Clone, Copy, Default)]
pub struct OutboxEventBus;

impl OutboxEventBus {
    pub fn new() -> Self {
        Self
    }
}

impl EventBus for OutboxEventBus {
    fn publish(&self, events: Vec<Box<dyn DomainEvent>>) -> Result<(), EventBusError> {
        match PgUnitOfWork::current() {
            Some(unit_of_work) => {
                unit_of_work.buffer_events(events);
                Ok(())
            }
            None => Err(EventBusError::DispatchError(
                "the outbox event bus can only publish inside a unit of work".to_string(),
            )),
        }
    }
}
