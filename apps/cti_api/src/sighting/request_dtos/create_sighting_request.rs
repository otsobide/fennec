use serde::Deserialize;

/// JSON request body for `POST /sightings`.
///
/// `observed_at` is a unix timestamp in seconds.
#[derive(Deserialize)]
pub struct CreateSightingRequest {
    pub id: String,
    pub ioc_id: String,
    pub source_id: String,
    pub observed_at: u64,
}
