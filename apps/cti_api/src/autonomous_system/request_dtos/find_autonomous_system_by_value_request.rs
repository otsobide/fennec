use serde::Deserialize;

/// Query string of `GET /autonomous-systems?value=<number>`.
#[derive(Deserialize)]
pub struct FindAutonomousSystemByValueRequest {
    pub value: String,
}
