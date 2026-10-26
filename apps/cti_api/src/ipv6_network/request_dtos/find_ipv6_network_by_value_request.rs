use serde::Deserialize;

/// Query string of `GET /ipv6-networks?value=<network>`.
#[derive(Deserialize)]
pub struct FindIpv6NetworkByValueRequest {
    pub value: String,
}
