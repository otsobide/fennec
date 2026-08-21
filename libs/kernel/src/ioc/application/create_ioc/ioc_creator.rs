//! Domain service for creating iocs.

use std::sync::Arc;

use shared_domain_events::domain::event_bus::EventBus;
use tracing::{debug, info};

use crate::ioc::domain::entities::ioc::Ioc;
use crate::ioc::domain::errors::ioc_repository_error::IocRepositoryError;
use crate::ioc::domain::events::create_ioc_created_event::create_ioc_created_event;
use crate::ioc::domain::repositories::ioc_repository::IocRepository;
use crate::ioc::domain::value_objects::ioc_created_at::IocCreatedAt;
use crate::ioc::domain::value_objects::ioc_id::IocId;
use crate::ioc::domain::value_objects::ioc_type::IocType;
use crate::ioc::domain::value_objects::ioc_updated_at::IocUpdatedAt;
use crate::ioc::domain::value_objects::ioc_value::IocValue;

/// Domain service that persists a new [`Ioc`] and publishes an
/// [`IocCreatedEvent`] via the event bus.
///
/// [`IocCreatedEvent`]: crate::ioc::domain::events::ioc_created_event::IocCreatedEvent
pub struct IocCreator {
    repository: Arc<dyn IocRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl IocCreator {
    pub fn new(repository: Arc<dyn IocRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            repository,
            event_bus,
        }
    }

    pub async fn execute(
        &self,
        id: IocId,
        ioc_type: IocType,
        value: IocValue,
    ) -> Result<(), IocRepositoryError> {
        let created_at = IocCreatedAt::now();
        let updated_at = IocUpdatedAt::from_system_time(created_at.value());

        let ioc = Ioc::new(id, ioc_type, value, created_at, updated_at);
        debug!(id = %ioc.id(), "Creating ioc");

        self.repository.save(&ioc).await?;

        let event = create_ioc_created_event(&ioc)?;
        self.event_bus
            .publish(vec![Box::new(event)])
            .map_err(|e| IocRepositoryError::Unexpected(e.to_string()))?;

        info!(id = %ioc.id(), "Ioc created");
        Ok(())
    }
}
