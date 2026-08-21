//! Factory function for [`IocDeletedEvent`].

use crate::ioc::domain::entities::ioc::Ioc;
use crate::ioc::domain::errors::ioc_repository_error::IocRepositoryError;
use crate::ioc::domain::events::ioc_deleted_event::IocDeletedEvent;

/// Creates an [`IocDeletedEvent`] from the deleted ioc.
pub fn create_ioc_deleted_event(ioc: &Ioc) -> Result<IocDeletedEvent, IocRepositoryError> {
    Ok(IocDeletedEvent::new(ioc.id().clone()))
}
