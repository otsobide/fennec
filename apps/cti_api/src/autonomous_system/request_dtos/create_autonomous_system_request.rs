use serde::Deserialize;

/// JSON request body for `POST /autonomous-systems`.
///
/// Unknown fields are rejected, so a body that tries to choose the id fails
/// with `400`: the id of an autonomous system is always derived from its value.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateAutonomousSystemRequest {
    pub value: String,
}
