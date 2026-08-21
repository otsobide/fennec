//! Response for the list-sightings-by-source use case.

use crate::sighting::application::find_sighting::find_sighting_response::{
    SightingEntry, SightingErrorEntry,
};

/// Response envelope returned by [`ListSightingsBySourceQueryHandler`].
///
/// `sightings` is always populated (empty vector if no results); `error` is
/// only set on unexpected repository failure.
///
/// [`ListSightingsBySourceQueryHandler`]: super::list_sightings_by_source_query_handler::ListSightingsBySourceQueryHandler
pub struct ListSightingsBySourceResponse {
    pub sightings: Vec<SightingEntry>,
    pub error: Option<SightingErrorEntry>,
}
