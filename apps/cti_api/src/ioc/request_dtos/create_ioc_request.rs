use serde::Deserialize;

/// JSON request body for `POST /iocs`.
#[derive(Deserialize)]
pub struct CreateIocRequest {
    pub id: String,
    pub ioc_type: String,
    pub value: String,
}
