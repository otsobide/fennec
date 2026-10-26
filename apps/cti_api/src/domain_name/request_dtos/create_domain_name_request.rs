use serde::Deserialize;

/// JSON request body for `POST /domain-names`.
///
/// Unknown fields are rejected, so a body that tries to choose the id fails
/// with `400`: the id of a domain name is always derived from its value.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateDomainNameRequest {
    pub value: String,
}
