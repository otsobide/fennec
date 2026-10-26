//! GET /autonomous-systems?value=<number> handler for the CTI API.

use actix_web::{get, web, HttpResponse, Responder};
use tracing::debug;

use kernel::autonomous_system::domain::value_objects::autonomous_system_id::AutonomousSystemId;
use kernel::autonomous_system::domain::value_objects::autonomous_system_value::AutonomousSystemValue;

use crate::autonomous_system::controllers::get::find_and_respond;
use crate::autonomous_system::request_dtos::find_autonomous_system_by_value_request::FindAutonomousSystemByValueRequest;
use crate::AppState;

/// Handles `GET /autonomous-systems?value=<number>`.
///
/// Derives the id from the value and looks it up, so any spelling of the
/// value (`64512`, `AS0.64512`) finds the same aggregate.
///
/// # Responses
///
/// - `200 OK` – the autonomous system, as in `GET /autonomous-systems/{id}`.
/// - `400 Bad Request` – the value is not a valid autonomous system.
/// - `404 Not Found` – the autonomous system is not registered.
/// - `500 Internal Server Error` – unexpected error.
#[get("/autonomous-systems")]
pub async fn handler(
    state: web::Data<AppState>,
    query: web::Query<FindAutonomousSystemByValueRequest>,
) -> impl Responder {
    debug!(value = %query.value, "GET /autonomous-systems?value=");

    let value = match AutonomousSystemValue::new(&query.value) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    find_and_respond(&state, AutonomousSystemId::from_value(&value)).await
}
