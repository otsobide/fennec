//! GET /autonomous-systems/{id} handler for the CTI API.

use actix_web::{get, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::autonomous_system::application::find_autonomous_system::find_autonomous_system_query::FindAutonomousSystemQuery;
use kernel::autonomous_system::application::find_autonomous_system::find_autonomous_system_response::FindAutonomousSystemResponse;
use kernel::autonomous_system::domain::value_objects::autonomous_system_id::AutonomousSystemId;

use crate::autonomous_system::response_dtos::autonomous_system_response::AutonomousSystemResponse;
use crate::AppState;

/// Handles `GET /autonomous-systems/{id}`.
///
/// # Responses
///
/// - `200 OK` – the autonomous system, as an [`AutonomousSystemResponse`].
/// - `400 Bad Request` – the path id is not a UUID v5.
/// - `404 Not Found` – no autonomous system is registered under the id.
/// - `500 Internal Server Error` – unexpected error.
#[get("/autonomous-systems/{id}")]
pub async fn handler(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    debug!(id = %id_str, "GET /autonomous-systems/{{id}}");

    let id = match AutonomousSystemId::new(&id_str) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    find_and_respond(&state, id).await
}

/// Dispatches a [`FindAutonomousSystemQuery`] and maps its response to HTTP.
///
/// Shared with the lookup by value, which derives the id first.
pub(crate) async fn find_and_respond(state: &AppState, id: AutonomousSystemId) -> HttpResponse {
    let query = FindAutonomousSystemQuery { id };

    match state.query_bus.ask(Box::new(query)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<FindAutonomousSystemResponse>()
                .expect("Unexpected response type from FindAutonomousSystemQueryHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "NotFound" => {
                        info!(id = %id, "Autonomous system not found");
                        HttpResponse::NotFound().body(error.message.clone())
                    }
                    _ => {
                        warn!(id = %id, error = %error.message, "Failed to find autonomous system");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else if let Some(ref entry) = response.autonomous_system {
                info!(id = %id, "Autonomous system found");
                HttpResponse::Ok().json(AutonomousSystemResponse::from_entry(entry))
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(e) => {
            warn!(id = %id, error = %e, "Failed to find autonomous system");
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}
