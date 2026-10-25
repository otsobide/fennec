//! Response for the list-sightings-by-observable use case.

use crate::sighting::application::find_sighting::find_sighting_response::{
    SightingEntry, SightingErrorEntry,
};

/// Response envelope returned by [`ListSightingsByObservableQueryHandler`].
///
/// `sightings` is always populated (empty vector if no results); `error` is
/// only set on unexpected repository failure.
///
/// [`ListSightingsByObservableQueryHandler`]: super::list_sightings_by_observable_query_handler::ListSightingsByObservableQueryHandler
pub struct ListSightingsByObservableResponse {
    pub sightings: Vec<SightingEntry>,
    pub error: Option<SightingErrorEntry>,
}
