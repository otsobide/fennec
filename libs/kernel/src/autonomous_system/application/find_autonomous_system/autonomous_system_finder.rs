//! Domain service for finding a single autonomous system.

use std::sync::Arc;

use tracing::{debug, warn};

use crate::autonomous_system::domain::entities::autonomous_system::AutonomousSystem;
use crate::autonomous_system::domain::errors::autonomous_system_repository_error::AutonomousSystemRepositoryError;
use crate::autonomous_system::domain::repositories::autonomous_system_repository::AutonomousSystemRepository;
use crate::autonomous_system::domain::value_objects::autonomous_system_id::AutonomousSystemId;

/// Domain service that looks up a single [`AutonomousSystem`] by id.
pub struct AutonomousSystemFinder {
    repository: Arc<dyn AutonomousSystemRepository>,
}

impl AutonomousSystemFinder {
    pub fn new(repository: Arc<dyn AutonomousSystemRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        id: AutonomousSystemId,
    ) -> Result<AutonomousSystem, AutonomousSystemRepositoryError> {
        debug!(id = %id, "Finding autonomous system");
        let autonomous_system = self.repository.find_by_id(&id).await?;

        autonomous_system.ok_or_else(|| {
            warn!(id = %id, "Autonomous system not found");
            AutonomousSystemRepositoryError::NotFound
        })
    }
}
