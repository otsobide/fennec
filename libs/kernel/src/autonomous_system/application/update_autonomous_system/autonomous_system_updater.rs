//! Domain service for describing an autonomous system.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info, warn};

use crate::autonomous_system::domain::errors::autonomous_system_repository_error::AutonomousSystemRepositoryError;
use crate::autonomous_system::domain::events::create_autonomous_system_updated_event::create_autonomous_system_updated_event;
use crate::autonomous_system::domain::repositories::autonomous_system_repository::AutonomousSystemRepository;
use crate::autonomous_system::domain::value_objects::autonomous_system_id::AutonomousSystemId;
use crate::autonomous_system::domain::value_objects::autonomous_system_name::AutonomousSystemName;
use crate::autonomous_system::domain::value_objects::autonomous_system_rir::AutonomousSystemRir;
use crate::autonomous_system::domain::value_objects::autonomous_system_updated_at::AutonomousSystemUpdatedAt;

/// Domain service that replaces the name and the registry of an
/// [`AutonomousSystem`] and publishes an [`AutonomousSystemUpdatedEvent`] via
/// the event bus.
///
/// [`AutonomousSystem`]: crate::autonomous_system::domain::entities::autonomous_system::AutonomousSystem
/// [`AutonomousSystemUpdatedEvent`]: crate::autonomous_system::domain::events::autonomous_system_updated_event::AutonomousSystemUpdatedEvent
pub struct AutonomousSystemUpdater {
    repository: Arc<dyn AutonomousSystemRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl AutonomousSystemUpdater {
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
        name: Option<AutonomousSystemName>,
        rir: Option<AutonomousSystemRir>,
    ) -> Result<(), AutonomousSystemRepositoryError> {
        debug!(id = %id, "Updating autonomous system");

        let previous = self.repository.find_by_id(&id).await?.ok_or_else(|| {
            warn!(id = %id, "Autonomous system not found for update");
            AutonomousSystemRepositoryError::NotFound
        })?;

        let updated = previous.describe(name, rir, AutonomousSystemUpdatedAt::now());
        self.repository.update(&updated).await?;

        let event = create_autonomous_system_updated_event(&updated, &previous)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| AutonomousSystemRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %id, "Autonomous system updated");
        Ok(())
    }
}
