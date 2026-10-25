//! Response for the list-sightings-by-ioc use case.

use crate::sighting::application::find_sighting::find_sighting_response::{
    SightingEntry, SightingErrorEntry,
};

/// Response envelope returned by [`ListSightingsByIocQueryHandler`].
///
/// `sightings` is always populated (empty vector if no results); `error` is
/// only set on unexpected repository failure.
///
/// [`ListSightingsByIocQueryHandler`]: super::list_sightings_by_ioc_query_handler::ListSightingsByIocQueryHandler
pub struct ListSightingsByIocResponse {
    pub sightings: Vec<SightingEntry>,
    pub error: Option<SightingErrorEntry>,
}
