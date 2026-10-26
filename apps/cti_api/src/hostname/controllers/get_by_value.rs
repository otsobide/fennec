//! GET /hostnames?value=<hostname> handler for the CTI API.

use actix_web::{get, web, HttpResponse, Responder};
use tracing::debug;

use kernel::hostname::domain::value_objects::hostname_id::HostnameId;
use kernel::hostname::domain::value_objects::hostname_value::HostnameValue;

use crate::hostname::controllers::get::find_and_respond;
use crate::hostname::request_dtos::find_hostname_by_value_request::FindHostnameByValueRequest;
use crate::AppState;

/// Handles `GET /hostnames?value=<hostname>`.
///
/// Derives the id from the value and looks it up, so any spelling of the
/// value (`printer-3f.lan`, `PRINTER-3F.LAN.`) finds the same aggregate.
///
/// # Responses
///
/// - `200 OK` – the hostname, as in `GET /hostnames/{id}`.
/// - `400 Bad Request` – the value is not a valid hostname.
/// - `404 Not Found` – the hostname is not registered.
/// - `500 Internal Server Error` – unexpected error.
#[get("/hostnames")]
pub async fn handler(
    state: web::Data<AppState>,
    query: web::Query<FindHostnameByValueRequest>,
) -> impl Responder {
    debug!(value = %query.value, "GET /hostnames?value=");

    let value = match HostnameValue::new(&query.value) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    find_and_respond(&state, HostnameId::from_value(&value)).await
}
