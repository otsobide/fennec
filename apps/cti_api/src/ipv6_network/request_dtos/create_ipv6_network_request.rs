use serde::Deserialize;

/// JSON request body for `POST /ipv6-networks`.
///
/// Unknown fields are rejected, so a body that tries to choose the id fails
/// with `400`: the id of an IPv6 network is always derived from its value.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateIpv6NetworkRequest {
    pub value: String,
}
