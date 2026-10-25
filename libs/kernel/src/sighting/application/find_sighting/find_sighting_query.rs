//! Query for finding a sighting by id.

use shared_cqrs::query::domain::query::Query;

use crate::sighting::domain::value_objects::sighting_id::SightingId;

/// Query that requests a single sighting by its [`SightingId`].
pub struct FindSightingQuery {
    pub id: SightingId,
}

impl Query for FindSightingQuery {}
