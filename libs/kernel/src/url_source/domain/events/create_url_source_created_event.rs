//! Factory function for [`UrlSourceCreatedEvent`].

use crate::url_source::domain::entities::url_source::UrlSource;
use crate::url_source::domain::errors::url_source_repository_error::UrlSourceRepositoryError;
use crate::url_source::domain::events::url_source_created_event::UrlSourceCreatedEvent;

/// Creates a [`UrlSourceCreatedEvent`] from the given url source.
pub fn create_url_source_created_event(
    url_source: &UrlSource,
) -> Result<UrlSourceCreatedEvent, UrlSourceRepositoryError> {
    Ok(UrlSourceCreatedEvent::new(
        url_source.id().clone(),
        url_source.url().clone(),
        url_source.format().clone(),
        *url_source.polling_interval(),
        url_source.created_at().clone(),
        url_source.updated_at().clone(),
    ))
}
