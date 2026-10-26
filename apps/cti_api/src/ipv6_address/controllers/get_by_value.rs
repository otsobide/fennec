//! GET /ipv6-addresses?value=<address> handler for the CTI API.

use actix_web::{get, web, HttpResponse, Responder};
use tracing::debug;

use kernel::ipv6_address::domain::value_objects::ipv6_address_id::Ipv6AddressId;
use kernel::ipv6_address::domain::value_objects::ipv6_address_value::Ipv6AddressValue;

use crate::ipv6_address::controllers::get::find_and_respond;
use crate::ipv6_address::request_dtos::find_ipv6_address_by_value_request::FindIpv6AddressByValueRequest;
use crate::AppState;

/// Handles `GET /ipv6-addresses?value=<address>`.
///
/// Derives the id from the value and looks it up, so any spelling of the
/// value (`fe80::1`, `FE80:0:0:0:0:0:0:1/128`) finds the same aggregate.
///
/// # Responses
///
/// - `200 OK` – the address, as in `GET /ipv6-addresses/{id}`.
/// - `400 Bad Request` – the value is not a valid IPv6 address.
/// - `404 Not Found` – the address is not registered.
/// - `500 Internal Server Error` – unexpected error.
#[get("/ipv6-addresses")]
pub async fn handler(
    state: web::Data<AppState>,
    query: web::Query<FindIpv6AddressByValueRequest>,
) -> impl Responder {
    debug!(value = %query.value, "GET /ipv6-addresses?value=");

    let value = match Ipv6AddressValue::new(&query.value) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    find_and_respond(&state, Ipv6AddressId::from_value(&value)).await
}
