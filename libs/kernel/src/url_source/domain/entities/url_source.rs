//! `UrlSource` aggregate root.
//!
//! Represents the URL-specific payload of a `Source` whose type is `Url`.
//! The identifier is shared 1:1 with the parent `Source` in the `kernel`
//! bounded context (i.e. `UrlSource.id == Source.id`).

use crate::url_source::domain::value_objects::url_source_created_at::UrlSourceCreatedAt;
use crate::url_source::domain::value_objects::url_source_format::UrlSourceFormat;
use crate::url_source::domain::value_objects::url_source_id::UrlSourceId;
use crate::url_source::domain::value_objects::url_source_polling_interval::UrlSourcePollingInterval;
use crate::url_source::domain::value_objects::url_source_updated_at::UrlSourceUpdatedAt;
use crate::url_source::domain::value_objects::url_source_url::UrlSourceUrl;

/// Aggregate root carrying the URL-specific attributes of a source.
#[derive(Clone)]
pub struct UrlSource {
    id: UrlSourceId,
    url: UrlSourceUrl,
    format: UrlSourceFormat,
    polling_interval: UrlSourcePollingInterval,
    created_at: UrlSourceCreatedAt,
    updated_at: UrlSourceUpdatedAt,
}

impl UrlSource {
    /// Creates a new `UrlSource` from its component value objects.
    pub fn new(
        id: UrlSourceId,
        url: UrlSourceUrl,
        format: UrlSourceFormat,
        polling_interval: UrlSourcePollingInterval,
        created_at: UrlSourceCreatedAt,
        updated_at: UrlSourceUpdatedAt,
    ) -> Self {
        Self {
            id,
            url,
            format,
            polling_interval,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> &UrlSourceId {
        &self.id
    }

    pub fn url(&self) -> &UrlSourceUrl {
        &self.url
    }

    pub fn format(&self) -> &UrlSourceFormat {
        &self.format
    }

    pub fn polling_interval(&self) -> &UrlSourcePollingInterval {
        &self.polling_interval
    }

    pub fn created_at(&self) -> &UrlSourceCreatedAt {
        &self.created_at
    }

    pub fn updated_at(&self) -> &UrlSourceUpdatedAt {
        &self.updated_at
    }
}
