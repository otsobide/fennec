use serde::Deserialize;

/// JSON request body for `PUT /url-sources/{id}`.
#[derive(Deserialize)]
pub struct UpdateUrlSourceRequest {
    pub url: String,
    pub format: String,
    pub polling_interval_seconds: u32,
}
