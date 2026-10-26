//! GET /ipv4-networks?value=<network> handler for the CTI API.

use actix_web::{get, web, HttpResponse, Responder};
use tracing::debug;

use kernel::ipv4_network::domain::value_objects::ipv4_network_id::Ipv4NetworkId;
use kernel::ipv4_network::domain::value_objects::ipv4_network_value::Ipv4NetworkValue;

use crate::ipv4_network::controllers::get::find_and_respond;
use crate::ipv4_network::request_dtos::find_ipv4_network_by_value_request::FindIpv4NetworkByValueRequest;
use crate::AppState;

/// Handles `GET /ipv4-networks?value=<network>`.
///
/// Derives the id from the value and looks it up, so any spelling of the
/// value (`10.0.0.0/8`, `  10.0.0.0/8`) finds the same aggregate.
///
/// # Responses
///
/// - `200 OK` – the network, as in `GET /ipv4-networks/{id}`.
/// - `400 Bad Request` – the value is not a valid IPv4 network.
/// - `404 Not Found` – the network is not registered.
/// - `500 Internal Server Error` – unexpected error.
#[get("/ipv4-networks")]
pub async fn handler(
    state: web::Data<AppState>,
    query: web::Query<FindIpv4NetworkByValueRequest>,
) -> impl Responder {
    debug!(value = %query.value, "GET /ipv4-networks?value=");

    let value = match Ipv4NetworkValue::new(&query.value) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    find_and_respond(&state, Ipv4NetworkId::from_value(&value)).await
}
