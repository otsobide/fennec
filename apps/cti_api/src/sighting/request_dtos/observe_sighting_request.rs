use serde::Deserialize;

/// JSON request body for `POST /sightings/{id}/observations`.
///
/// `observed_at` is a unix timestamp in seconds.
#[derive(Deserialize)]
pub struct ObserveSightingRequest {
    pub observed_at: u64,
}
