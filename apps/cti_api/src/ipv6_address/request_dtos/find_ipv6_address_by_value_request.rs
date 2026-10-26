use serde::Deserialize;

/// Query string of `GET /ipv6-addresses?value=<address>`.
#[derive(Deserialize)]
pub struct FindIpv6AddressByValueRequest {
    pub value: String,
}
