//! Domain service for registering autonomous systems.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info};

use crate::autonomous_system::domain::entities::autonomous_system::AutonomousSystem;
use crate::autonomous_system::domain::errors::autonomous_system_repository_error::AutonomousSystemRepositoryError;
use crate::autonomous_system::domain::events::create_autonomous_system_created_event::create_autonomous_system_created_event;
use crate::autonomous_system::domain::repositories::autonomous_system_repository::AutonomousSystemRepository;
use crate::autonomous_system::domain::value_objects::autonomous_system_created_at::AutonomousSystemCreatedAt;
use crate::autonomous_system::domain::value_objects::autonomous_system_value::AutonomousSystemValue;

/// Domain service that persists a new [`AutonomousSystem`] and publishes an
/// [`AutonomousSystemCreatedEvent`] via the event bus.
///
/// The id is derived from the value and `created_at` is generated here, so
/// the caller provides only the autonomous system.
///
/// [`AutonomousSystemCreatedEvent`]: crate::autonomous_system::domain::events::autonomous_system_created_event::AutonomousSystemCreatedEvent
pub struct AutonomousSystemCreator {
    repository: Arc<dyn AutonomousSystemRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl AutonomousSystemCreator {
    pub fn new(
        repository: Arc<dyn AutonomousSystemRepository>,
        event_bus: Arc<dyn EventBus>,
    ) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    pub async fn execute(
        &self,
        value: AutonomousSystemValue,
    ) -> Result<(), AutonomousSystemRepositoryError> {
        let autonomous_system = AutonomousSystem::new(value, AutonomousSystemCreatedAt::now());
        debug!(id = %autonomous_system.id(), value = %autonomous_system.value(), "Creating autonomous system");

        self.repository.save(&autonomous_system).await?;

        let event = create_autonomous_system_created_event(&autonomous_system)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| AutonomousSystemRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %autonomous_system.id(), "Autonomous system created");
        Ok(())
    }
}
