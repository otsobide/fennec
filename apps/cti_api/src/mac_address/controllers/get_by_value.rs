//! GET /mac-addresses?value=<address> handler for the CTI API.

use actix_web::{get, web, HttpResponse, Responder};
use tracing::debug;

use kernel::mac_address::domain::value_objects::mac_address_id::MacAddressId;
use kernel::mac_address::domain::value_objects::mac_address_value::MacAddressValue;

use crate::mac_address::controllers::get::find_and_respond;
use crate::mac_address::request_dtos::find_mac_address_by_value_request::FindMacAddressByValueRequest;
use crate::AppState;

/// Handles `GET /mac-addresses?value=<address>`.
///
/// Derives the id from the value and looks it up, so any spelling of the
/// value (`02:00:5e:10:00:00`, `02005E100000`) finds the same aggregate.
///
/// # Responses
///
/// - `200 OK` – the address, as in `GET /mac-addresses/{id}`.
/// - `400 Bad Request` – the value is not a valid MAC address.
/// - `404 Not Found` – the address is not registered.
/// - `500 Internal Server Error` – unexpected error.
#[get("/mac-addresses")]
pub async fn handler(
    state: web::Data<AppState>,
    query: web::Query<FindMacAddressByValueRequest>,
) -> impl Responder {
    debug!(value = %query.value, "GET /mac-addresses?value=");

    let value = match MacAddressValue::new(&query.value) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    find_and_respond(&state, MacAddressId::from_value(&value)).await
}
