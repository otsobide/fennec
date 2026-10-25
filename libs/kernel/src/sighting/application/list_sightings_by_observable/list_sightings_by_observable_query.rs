//! Query for listing the sightings of a given observable.

use shared_cqrs::query::domain::query::Query;

use crate::sighting::domain::value_objects::sighting_observable_id::SightingObservableId;
use crate::sighting::domain::value_objects::sighting_observable_kind::SightingObservableKind;

/// Query that requests every sighting of the given observable.
pub struct ListSightingsByObservableQuery {
    pub observable_kind: SightingObservableKind,
    pub observable_id: SightingObservableId,
}

impl Query for ListSightingsByObservableQuery {}
