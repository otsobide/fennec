use serde::Deserialize;

/// Query string of `GET /ipv4-addresses?value=<address>`.
#[derive(Deserialize)]
pub struct FindIpv4AddressByValueRequest {
    pub value: String,
}
