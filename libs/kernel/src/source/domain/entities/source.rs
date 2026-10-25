//! `Source` aggregate root.
//!
//! A `Source` represents the origin of intelligence (a feed, a producer, an
//! external system). It is identified by an externally-provided UUID and
//! carries only the fields common to every kind of source; type-specific
//! payloads (e.g. the URL itself for a URL source) live in downstream
//! bounded contexts.

use crate::source::domain::value_objects::source_created_at::SourceCreatedAt;
use crate::source::domain::value_objects::source_description::SourceDescription;
use crate::source::domain::value_objects::source_id::SourceId;
use crate::source::domain::value_objects::source_status::SourceStatus;
use crate::source::domain::value_objects::source_type::SourceType;
use crate::source::domain::value_objects::source_updated_at::SourceUpdatedAt;

/// Aggregate root representing a source of intelligence.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Source {
    id: SourceId,
    source_type: SourceType,
    status: SourceStatus,
    description: SourceDescription,
    created_at: SourceCreatedAt,
    updated_at: SourceUpdatedAt,
}

impl Source {
    /// Creates a new `Source` from its component value objects.
    pub fn new(
        id: SourceId,
        source_type: SourceType,
        status: SourceStatus,
        description: SourceDescription,
        created_at: SourceCreatedAt,
        updated_at: SourceUpdatedAt,
    ) -> Self {
        Self {
            id,
            source_type,
            status,
            description,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> &SourceId {
        &self.id
    }

    pub fn source_type(&self) -> &SourceType {
        &self.source_type
    }

    pub fn status(&self) -> &SourceStatus {
        &self.status
    }

    pub fn description(&self) -> &SourceDescription {
        &self.description
    }

    pub fn created_at(&self) -> &SourceCreatedAt {
        &self.created_at
    }

    pub fn updated_at(&self) -> &SourceUpdatedAt {
        &self.updated_at
    }
}
