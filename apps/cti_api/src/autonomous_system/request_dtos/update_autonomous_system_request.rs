use serde::Deserialize;

/// JSON request body for `PUT /autonomous-systems/{id}`.
///
/// It replaces the whole description: a missing or `null` field clears it.
/// Unknown fields are rejected, so a body that tries to change the number
/// fails with `400`: the number is the identity.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateAutonomousSystemRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub rir: Option<String>,
}
