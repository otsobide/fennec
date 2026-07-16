//! Domain service for deleting an ioc.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info, warn};

use crate::ioc::domain::errors::ioc_repository_error::IocRepositoryError;
use crate::ioc::domain::events::create_ioc_deleted_event::create_ioc_deleted_event;
use crate::ioc::domain::repositories::ioc_repository::IocRepository;
use crate::ioc::domain::value_objects::ioc_id::IocId;

/// Domain service that deletes an [`Ioc`] and publishes an
/// [`IocDeletedEvent`] via the event bus.
///
/// [`Ioc`]: crate::ioc::domain::entities::ioc::Ioc
/// [`IocDeletedEvent`]: crate::ioc::domain::events::ioc_deleted_event::IocDeletedEvent
pub struct IocDeleter {
    repository: Arc<dyn IocRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl IocDeleter {
    pub fn new(repository: Arc<dyn IocRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self { repository, event_bus }
    }

    pub async fn execute(&self, id: IocId) -> Result<(), IocRepositoryError> {
        debug!(id = %id, "Deleting ioc");

        let ioc = self
            .repository
            .find_by_id(&id)
            .await?
            .ok_or_else(|| {
                warn!(id = %id, "Ioc not found for deletion");
                IocRepositoryError::NotFound
            })?;

        self.repository.delete(&id).await?;

        let event = create_ioc_deleted_event(&ioc)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| IocRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %id, "Ioc deleted");
        Ok(())
    }
}
