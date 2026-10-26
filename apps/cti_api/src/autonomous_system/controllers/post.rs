//! POST /autonomous-systems handler for the CTI API.

use actix_web::{post, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::autonomous_system::application::create_autonomous_system::create_autonomous_system_command::CreateAutonomousSystemCommand;
use kernel::autonomous_system::application::create_autonomous_system::create_autonomous_system_response::CreateAutonomousSystemResponse;
use kernel::autonomous_system::domain::value_objects::autonomous_system_id::AutonomousSystemId;
use kernel::autonomous_system::domain::value_objects::autonomous_system_value::AutonomousSystemValue;

use crate::autonomous_system::request_dtos::create_autonomous_system_request::CreateAutonomousSystemRequest;
use crate::autonomous_system::response_dtos::autonomous_system_reference_response::AutonomousSystemReferenceResponse;
use crate::AppState;

/// Handles `POST /autonomous-systems`.
///
/// # Responses
///
/// - `201 Created` – the autonomous system was registered; `Location` points at it and
///   the body carries its `id` and `stix_id`.
/// - `400 Bad Request` – the value is not a valid autonomous system, or the
///   body has fields other than `value`.
/// - `409 Conflict` – the autonomous system is already registered; the body carries its
///   `id` and `stix_id`.
/// - `500 Internal Server Error` – unexpected error.
#[post("/autonomous-systems")]
pub async fn handler(
    state: web::Data<AppState>,
    body: web::Json<CreateAutonomousSystemRequest>,
) -> impl Responder {
    debug!(value = %body.value, "POST /autonomous-systems");

    let value = match AutonomousSystemValue::new(&body.value) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };
    let reference =
        AutonomousSystemReferenceResponse::from_id(&AutonomousSystemId::from_value(&value));

    let command = CreateAutonomousSystemCommand { value };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<CreateAutonomousSystemResponse>()
                .expect("Unexpected response type from CreateAutonomousSystemCommandHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "AlreadyExists" => {
                        warn!(id = %reference.id, "Autonomous system already exists");
                        HttpResponse::Conflict().json(reference)
                    }
                    _ => {
                        warn!(id = %reference.id, error = %error.message, "Failed to create autonomous system");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else {
                info!(id = %reference.id, "Autonomous system created");
                HttpResponse::Created()
                    .insert_header(("Location", format!("/autonomous-systems/{}", reference.id)))
                    .json(reference)
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
