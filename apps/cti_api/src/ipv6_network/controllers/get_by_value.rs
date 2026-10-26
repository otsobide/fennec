//! GET /ipv6-networks?value=<network> handler for the CTI API.

use actix_web::{get, web, HttpResponse, Responder};
use tracing::debug;

use kernel::ipv6_network::domain::value_objects::ipv6_network_id::Ipv6NetworkId;
use kernel::ipv6_network::domain::value_objects::ipv6_network_value::Ipv6NetworkValue;

use crate::ipv6_network::controllers::get::find_and_respond;
use crate::ipv6_network::request_dtos::find_ipv6_network_by_value_request::FindIpv6NetworkByValueRequest;
use crate::AppState;

/// Handles `GET /ipv6-networks?value=<network>`.
///
/// Derives the id from the value and looks it up, so any spelling of the
/// value (`fd12:3456::/32`, `FD12:3456:0:0:0:0:0:0/32`) finds the same aggregate.
///
/// # Responses
///
/// - `200 OK` – the network, as in `GET /ipv6-networks/{id}`.
/// - `400 Bad Request` – the value is not a valid IPv6 network.
/// - `404 Not Found` – the network is not registered.
/// - `500 Internal Server Error` – unexpected error.
#[get("/ipv6-networks")]
pub async fn handler(
    state: web::Data<AppState>,
    query: web::Query<FindIpv6NetworkByValueRequest>,
) -> impl Responder {
    debug!(value = %query.value, "GET /ipv6-networks?value=");

    let value = match Ipv6NetworkValue::new(&query.value) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    find_and_respond(&state, Ipv6NetworkId::from_value(&value)).await
}
