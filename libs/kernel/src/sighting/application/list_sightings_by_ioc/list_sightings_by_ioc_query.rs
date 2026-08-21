//! Query for listing sightings for a given ioc.

use shared_cqrs::query::domain::query::Query;

use crate::sighting::domain::value_objects::sighting_ioc_id::SightingIocId;

/// Query that requests every sighting that references the given ioc id.
pub struct ListSightingsByIocQuery {
    pub ioc_id: SightingIocId,
}

impl Query for ListSightingsByIocQuery {}
