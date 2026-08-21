//! Query for listing sightings for a given source.

use shared_cqrs::query::domain::query::Query;

use crate::sighting::domain::value_objects::sighting_source_id::SightingSourceId;

/// Query that requests every sighting reported by the given source id.
pub struct ListSightingsBySourceQuery {
    pub source_id: SightingSourceId,
}

impl Query for ListSightingsBySourceQuery {}
