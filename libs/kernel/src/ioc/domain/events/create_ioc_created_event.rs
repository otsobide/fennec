//! Factory function for [`IocCreatedEvent`].

use crate::ioc::domain::entities::ioc::Ioc;
use crate::ioc::domain::errors::ioc_repository_error::IocRepositoryError;
use crate::ioc::domain::events::ioc_created_event::IocCreatedEvent;

/// Creates an [`IocCreatedEvent`] from the given ioc.
pub fn create_ioc_created_event(ioc: &Ioc) -> Result<IocCreatedEvent, IocRepositoryError> {
    Ok(IocCreatedEvent::new(
        ioc.id().clone(),
        ioc.ioc_type().clone(),
        ioc.value().clone(),
        ioc.created_at().clone(),
        ioc.updated_at().clone(),
    ))
}
