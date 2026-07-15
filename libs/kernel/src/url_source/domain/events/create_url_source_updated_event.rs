//! Factory function for [`UrlSourceUpdatedEvent`].

use crate::url_source::domain::entities::url_source::UrlSource;
use crate::url_source::domain::errors::url_source_repository_error::UrlSourceRepositoryError;
use crate::url_source::domain::events::url_source_updated_event::UrlSourceUpdatedEvent;

/// Creates a [`UrlSourceUpdatedEvent`] from the updated and previous url sources.
pub fn create_url_source_updated_event(
    updated: &UrlSource,
    previous: &UrlSource,
) -> Result<UrlSourceUpdatedEvent, UrlSourceRepositoryError> {
    Ok(UrlSourceUpdatedEvent::new(
        updated.id().clone(),
        updated.url().clone(),
        previous.url().clone(),
        updated.format().clone(),
        previous.format().clone(),
        *updated.polling_interval(),
        *previous.polling_interval(),
        updated.updated_at().clone(),
    ))
}
