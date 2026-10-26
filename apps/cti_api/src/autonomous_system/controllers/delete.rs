//! DELETE /autonomous-systems/{id} handler for the CTI API.

use actix_web::{delete, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::autonomous_system::application::delete_autonomous_system::delete_autonomous_system_command::DeleteAutonomousSystemCommand;
use kernel::autonomous_system::application::delete_autonomous_system::delete_autonomous_system_response::DeleteAutonomousSystemResponse;
use kernel::autonomous_system::domain::value_objects::autonomous_system_id::AutonomousSystemId;

use crate::AppState;

/// Handles `DELETE /autonomous-systems/{id}`.
///
/// # Responses
///
/// - `204 No Content` – the autonomous system was deleted.
/// - `400 Bad Request` – the path id is not a UUID v5.
/// - `404 Not Found` – no autonomous system is registered under the id.
/// - `500 Internal Server Error` – unexpected error.
#[delete("/autonomous-systems/{id}")]
pub async fn handler(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    debug!(id = %id_str, "DELETE /autonomous-systems/{{id}}");

    let id = match AutonomousSystemId::new(&id_str) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    let command = DeleteAutonomousSystemCommand { id };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<DeleteAutonomousSystemResponse>()
                .expect("Unexpected response type from DeleteAutonomousSystemCommandHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "NotFound" => {
                        warn!(id = %id_str, "Autonomous system not found for deletion");
                        HttpResponse::NotFound().body(error.message.clone())
                    }
                    _ => {
                        warn!(id = %id_str, error = %error.message, "Failed to delete autonomous system");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else {
                info!(id = %id_str, "Autonomous system deleted");
                HttpResponse::NoContent().finish()
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
