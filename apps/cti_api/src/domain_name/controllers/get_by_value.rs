//! GET /domain-names?value=<name> handler for the CTI API.

use actix_web::{get, web, HttpResponse, Responder};
use tracing::debug;

use kernel::domain_name::domain::value_objects::domain_name_id::DomainNameId;
use kernel::domain_name::domain::value_objects::domain_name_value::DomainNameValue;

use crate::domain_name::controllers::get::find_and_respond;
use crate::domain_name::request_dtos::find_domain_name_by_value_request::FindDomainNameByValueRequest;
use crate::AppState;

/// Handles `GET /domain-names?value=<name>`.
///
/// Derives the id from the value and looks it up, so any spelling of the
/// value (`update.example-cdn.net`, `UPDATE.Example-CDN.net.`) finds the same aggregate.
///
/// # Responses
///
/// - `200 OK` – the name, as in `GET /domain-names/{id}`.
/// - `400 Bad Request` – the value is not a valid domain name.
/// - `404 Not Found` – the name is not registered.
/// - `500 Internal Server Error` – unexpected error.
#[get("/domain-names")]
pub async fn handler(
    state: web::Data<AppState>,
    query: web::Query<FindDomainNameByValueRequest>,
) -> impl Responder {
    debug!(value = %query.value, "GET /domain-names?value=");

    let value = match DomainNameValue::new(&query.value) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    find_and_respond(&state, DomainNameId::from_value(&value)).await
}
