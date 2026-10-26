use serde::Deserialize;

/// JSON request body for `POST /mac-addresses`.
///
/// Unknown fields are rejected, so a body that tries to choose the id fails
/// with `400`: the id of a MAC address is always derived from its value.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateMacAddressRequest {
    pub value: String,
}
