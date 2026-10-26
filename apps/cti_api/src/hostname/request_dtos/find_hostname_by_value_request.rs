use serde::Deserialize;

/// Query string of `GET /hostnames?value=<hostname>`.
#[derive(Deserialize)]
pub struct FindHostnameByValueRequest {
    pub value: String,
}
