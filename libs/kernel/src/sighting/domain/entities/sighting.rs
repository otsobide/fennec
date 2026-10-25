//! `Sighting` aggregate root.
//!
//! Represents an observation: a specific `Source` reported a specific
//! observable. One sighting exists per `(observable_kind, observable_id,
//! source_id)` triple, so an observable reported by N sources produces N
//! sightings. Observables and sources are referenced by identifier only: the
//! `sighting` module imports neither the observable modules nor `source`.
//! The observable reference carries the module kind next to the id because
//! ids of different kinds can share a UUID.

use std::time::SystemTime;

use crate::sighting::domain::value_objects::sighting_count::SightingCount;
use crate::sighting::domain::value_objects::sighting_created_at::SightingCreatedAt;
use crate::sighting::domain::value_objects::sighting_first_seen::SightingFirstSeen;
use crate::sighting::domain::value_objects::sighting_id::SightingId;
use crate::sighting::domain::value_objects::sighting_last_seen::SightingLastSeen;
use crate::sighting::domain::value_objects::sighting_observable_id::SightingObservableId;
use crate::sighting::domain::value_objects::sighting_observable_kind::SightingObservableKind;
use crate::sighting::domain::value_objects::sighting_source_id::SightingSourceId;
use crate::sighting::domain::value_objects::sighting_updated_at::SightingUpdatedAt;

/// Aggregate root modelling the observations of one observable by one source.
#[derive(Clone)]
pub struct Sighting {
    id: SightingId,
    observable_kind: SightingObservableKind,
    observable_id: SightingObservableId,
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
        observable_kind: SightingObservableKind,
        observable_id: SightingObservableId,
        source_id: SightingSourceId,
        first_seen: SightingFirstSeen,
        last_seen: SightingLastSeen,
        count: SightingCount,
        created_at: SightingCreatedAt,
        updated_at: SightingUpdatedAt,
    ) -> Self {
        Self {
            id,
            observable_kind,
            observable_id,
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

    pub fn observable_kind(&self) -> &SightingObservableKind {
        &self.observable_kind
    }

    pub fn observable_id(&self) -> &SightingObservableId {
        &self.observable_id
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
            observable_kind: self.observable_kind.clone(),
            observable_id: self.observable_id,
            source_id: self.source_id.clone(),
            first_seen: self.first_seen.clone(),
            last_seen: SightingLastSeen::from_system_time(new_last_seen),
            count: self.count.incremented(),
            created_at: self.created_at.clone(),
            updated_at,
        }
    }
}
