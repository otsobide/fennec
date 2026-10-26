//! Domain service for deleting an autonomous system.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info, warn};

use crate::autonomous_system::domain::errors::autonomous_system_repository_error::AutonomousSystemRepositoryError;
use crate::autonomous_system::domain::events::create_autonomous_system_deleted_event::create_autonomous_system_deleted_event;
use crate::autonomous_system::domain::repositories::autonomous_system_repository::AutonomousSystemRepository;
use crate::autonomous_system::domain::value_objects::autonomous_system_id::AutonomousSystemId;

/// Domain service that deletes an [`AutonomousSystem`] and publishes an
/// [`AutonomousSystemDeletedEvent`] via the event bus.
///
/// [`AutonomousSystem`]: crate::autonomous_system::domain::entities::autonomous_system::AutonomousSystem
/// [`AutonomousSystemDeletedEvent`]: crate::autonomous_system::domain::events::autonomous_system_deleted_event::AutonomousSystemDeletedEvent
pub struct AutonomousSystemDeleter {
    repository: Arc<dyn AutonomousSystemRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl AutonomousSystemDeleter {
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
        id: AutonomousSystemId,
    ) -> Result<(), AutonomousSystemRepositoryError> {
        debug!(id = %id, "Deleting autonomous system");

        let autonomous_system = self.repository.find_by_id(&id).await?.ok_or_else(|| {
            warn!(id = %id, "Autonomous system not found for deletion");
            AutonomousSystemRepositoryError::NotFound
        })?;

        self.repository.delete(&id).await?;

        let event = create_autonomous_system_deleted_event(&autonomous_system)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| AutonomousSystemRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %id, "Autonomous system deleted");
        Ok(())
    }
}
