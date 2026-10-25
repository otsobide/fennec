//! GET /ipv4-addresses?value=<address> handler for the CTI API.

use actix_web::{get, web, HttpResponse, Responder};
use tracing::debug;

use kernel::ipv4_address::domain::value_objects::ipv4_address_id::Ipv4AddressId;
use kernel::ipv4_address::domain::value_objects::ipv4_address_value::Ipv4AddressValue;

use crate::ipv4_address::controllers::get::find_and_respond;
use crate::ipv4_address::request_dtos::find_ipv4_address_by_value_request::FindIpv4AddressByValueRequest;
use crate::AppState;

/// Handles `GET /ipv4-addresses?value=<address>`.
///
/// Derives the id from the value and looks it up, so any spelling of the
/// address (`198.51.100.7`, `198.51.100.7/32`) finds the same aggregate.
///
/// # Responses
///
/// - `200 OK` – the address, as in `GET /ipv4-addresses/{id}`.
/// - `400 Bad Request` – the value is not a single IPv4 host address.
/// - `404 Not Found` – the address is not registered.
/// - `500 Internal Server Error` – unexpected error.
#[get("/ipv4-addresses")]
pub async fn handler(
    state: web::Data<AppState>,
    query: web::Query<FindIpv4AddressByValueRequest>,
) -> impl Responder {
    debug!(value = %query.value, "GET /ipv4-addresses?value=");

    let value = match Ipv4AddressValue::new(&query.value) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    find_and_respond(&state, Ipv4AddressId::from_value(&value)).await
}
