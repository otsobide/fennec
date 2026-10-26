//! PUT /autonomous-systems/{id} handler for the CTI API.

use actix_web::{put, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::autonomous_system::application::update_autonomous_system::update_autonomous_system_command::UpdateAutonomousSystemCommand;
use kernel::autonomous_system::application::update_autonomous_system::update_autonomous_system_response::UpdateAutonomousSystemResponse;
use kernel::autonomous_system::domain::value_objects::autonomous_system_id::AutonomousSystemId;
use kernel::autonomous_system::domain::value_objects::autonomous_system_name::AutonomousSystemName;
use kernel::autonomous_system::domain::value_objects::autonomous_system_rir::AutonomousSystemRir;

use crate::autonomous_system::request_dtos::update_autonomous_system_request::UpdateAutonomousSystemRequest;
use crate::AppState;

/// Handles `PUT /autonomous-systems/{id}`: replaces the name and the
/// registry of a registered autonomous system.
///
/// # Responses
///
/// - `200 OK` – the description was replaced.
/// - `400 Bad Request` – the path id is not a UUID v5, the name or the
///   registry is invalid, or the body has an unknown field.
/// - `404 Not Found` – no autonomous system is registered under the id.
/// - `500 Internal Server Error` – unexpected error.
#[put("/autonomous-systems/{id}")]
pub async fn handler(
    state: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<UpdateAutonomousSystemRequest>,
) -> impl Responder {
    let id_str = path.into_inner();
    debug!(id = %id_str, "PUT /autonomous-systems/{{id}}");

    let id = match AutonomousSystemId::new(&id_str) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };
    let name = match body
        .name
        .as_deref()
        .map(AutonomousSystemName::new)
        .transpose()
    {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };
    let rir = match body
        .rir
        .as_deref()
        .map(AutonomousSystemRir::new)
        .transpose()
    {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    let command = UpdateAutonomousSystemCommand { id, name, rir };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<UpdateAutonomousSystemResponse>()
                .expect("Unexpected response type from UpdateAutonomousSystemCommandHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "NotFound" => {
                        warn!(id = %id_str, "Autonomous system not found for update");
                        HttpResponse::NotFound().body(error.message.clone())
                    }
                    _ => {
                        warn!(id = %id_str, error = %error.message, "Failed to update autonomous system");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else {
                info!(id = %id_str, "Autonomous system updated");
                HttpResponse::Ok().finish()
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
