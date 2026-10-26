use serde::Deserialize;

/// Query string of `GET /domain-names?value=<name>`.
#[derive(Deserialize)]
pub struct FindDomainNameByValueRequest {
    pub value: String,
}
