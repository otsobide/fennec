//! GET /email-addresses?value=<address> handler for the CTI API.

use actix_web::{get, web, HttpResponse, Responder};
use tracing::debug;

use kernel::email_address::domain::value_objects::email_address_id::EmailAddressId;
use kernel::email_address::domain::value_objects::email_address_value::EmailAddressValue;

use crate::email_address::controllers::get::find_and_respond;
use crate::email_address::request_dtos::find_email_address_by_value_request::FindEmailAddressByValueRequest;
use crate::AppState;

/// Handles `GET /email-addresses?value=<address>`.
///
/// Derives the id from the value and looks it up, so any spelling of the
/// value (`first.last+tag@mail.example.org`, `First.Last+Tag@MAIL.example.org`) finds the same aggregate.
///
/// # Responses
///
/// - `200 OK` – the address, as in `GET /email-addresses/{id}`.
/// - `400 Bad Request` – the value is not a valid email address.
/// - `404 Not Found` – the address is not registered.
/// - `500 Internal Server Error` – unexpected error.
#[get("/email-addresses")]
pub async fn handler(
    state: web::Data<AppState>,
    query: web::Query<FindEmailAddressByValueRequest>,
) -> impl Responder {
    debug!(value = %query.value, "GET /email-addresses?value=");

    let value = match EmailAddressValue::new(&query.value) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    find_and_respond(&state, EmailAddressId::from_value(&value)).await
}
