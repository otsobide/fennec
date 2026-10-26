use serde::Deserialize;

/// Query string of `GET /mac-addresses?value=<address>`.
#[derive(Deserialize)]
pub struct FindMacAddressByValueRequest {
    pub value: String,
}
