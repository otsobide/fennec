use serde::Deserialize;

/// Query string of `GET /email-addresses?value=<address>`.
#[derive(Deserialize)]
pub struct FindEmailAddressByValueRequest {
    pub value: String,
}
