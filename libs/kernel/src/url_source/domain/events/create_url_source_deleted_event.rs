//! Factory function for [`UrlSourceDeletedEvent`].

use crate::url_source::domain::entities::url_source::UrlSource;
use crate::url_source::domain::errors::url_source_repository_error::UrlSourceRepositoryError;
use crate::url_source::domain::events::url_source_deleted_event::UrlSourceDeletedEvent;

/// Creates a [`UrlSourceDeletedEvent`] from the deleted url source.
pub fn create_url_source_deleted_event(
    url_source: &UrlSource,
) -> Result<UrlSourceDeletedEvent, UrlSourceRepositoryError> {
    Ok(UrlSourceDeletedEvent::new(url_source.id().clone()))
}
