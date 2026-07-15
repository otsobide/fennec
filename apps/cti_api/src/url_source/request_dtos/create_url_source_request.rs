use serde::Deserialize;

/// JSON request body for `POST /url-sources`.
#[derive(Deserialize)]
pub struct CreateUrlSourceRequest {
    pub id: String,
    pub url: String,
    pub format: String,
    pub polling_interval_seconds: u32,
}
