use serde::Deserialize;

/// Query string of `GET /ipv4-networks?value=<network>`.
#[derive(Deserialize)]
pub struct FindIpv4NetworkByValueRequest {
    pub value: String,
}
