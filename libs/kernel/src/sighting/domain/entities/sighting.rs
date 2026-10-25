//! `Sighting` aggregate root.
//!
//! Represents an observation: a specific `Source` reported a specific `Ioc`.
//! One row per `(ioc_id, source_id)` pair — an IoC reported by N sources
//! produces N sightings. References to `Ioc` and `Source` are by identifier
//! only; the `sighting` bounded context does not import the `ioc` or `kernel`
//! crates.

use std::time::SystemTime;

use crate::sighting::domain::value_objects::sighting_count::SightingCount;
use crate::sighting::domain::value_objects::sighting_created_at::SightingCreatedAt;
use crate::sighting::domain::value_objects::sighting_first_seen::SightingFirstSeen;
use crate::sighting::domain::value_objects::sighting_id::SightingId;
use crate::sighting::domain::value_objects::sighting_ioc_id::SightingIocId;
use crate::sighting::domain::value_objects::sighting_last_seen::SightingLastSeen;
use crate::sighting::domain::value_objects::sighting_source_id::SightingSourceId;
use crate::sighting::domain::value_objects::sighting_updated_at::SightingUpdatedAt;

/// Aggregate root modelling a single observation of an ioc by a source.
#[derive(Clone)]
pub struct Sighting {
    id: SightingId,
    ioc_id: SightingIocId,
    source_id: SightingSourceId,
    first_seen: SightingFirstSeen,
    last_seen: SightingLastSeen,
    count: SightingCount,
    created_at: SightingCreatedAt,
    updated_at: SightingUpdatedAt,
}

impl Sighting {
    /// Creates a new `Sighting` from its component value objects.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: SightingId,
        ioc_id: SightingIocId,
        source_id: SightingSourceId,
        first_seen: SightingFirstSeen,
        last_seen: SightingLastSeen,
        count: SightingCount,
        created_at: SightingCreatedAt,
        updated_at: SightingUpdatedAt,
    ) -> Self {
        Self {
            id,
            ioc_id,
            source_id,
            first_seen,
            last_seen,
            count,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> &SightingId {
        &self.id
    }

    pub fn ioc_id(&self) -> &SightingIocId {
        &self.ioc_id
    }

    pub fn source_id(&self) -> &SightingSourceId {
        &self.source_id
    }

    pub fn first_seen(&self) -> &SightingFirstSeen {
        &self.first_seen
    }

    pub fn last_seen(&self) -> &SightingLastSeen {
        &self.last_seen
    }

    pub fn count(&self) -> &SightingCount {
        &self.count
    }

    pub fn created_at(&self) -> &SightingCreatedAt {
        &self.created_at
    }

    pub fn updated_at(&self) -> &SightingUpdatedAt {
        &self.updated_at
    }

    /// Returns a cloned `Sighting` reflecting a new observation:
    ///
    /// - `count` is incremented by 1.
    /// - `last_seen` is set to `max(self.last_seen, observed_at)`.
    /// - `updated_at` is set to the provided timestamp.
    ///
    /// `first_seen` and `created_at` are preserved.
    pub fn observe(&self, observed_at: SystemTime, updated_at: SightingUpdatedAt) -> Sighting {
        let previous_last_seen = self.last_seen.value();
        let new_last_seen = if observed_at > previous_last_seen {
            observed_at
        } else {
            previous_last_seen
        };

        Sighting {
            id: self.id.clone(),
            ioc_id: self.ioc_id.clone(),
            source_id: self.source_id.clone(),
            first_seen: self.first_seen.clone(),
            last_seen: SightingLastSeen::from_system_time(new_last_seen),
            count: self.count.incremented(),
            created_at: self.created_at.clone(),
            updated_at,
        }
    }
}
