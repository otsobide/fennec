//! POST /sightings/{id}/observations handler for the CTI API.

use std::time::{Duration, UNIX_EPOCH};

use actix_web::{post, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::sighting::application::observe_sighting::observe_sighting_command::ObserveSightingCommand;
use kernel::sighting::application::observe_sighting::observe_sighting_response::ObserveSightingResponse;
use kernel::sighting::domain::value_objects::sighting_id::SightingId;

use crate::sighting::request_dtos::observe_sighting_request::ObserveSightingRequest;
use crate::AppState;

#[post("/sightings/{id}/observations")]
pub async fn handler(
    state: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<ObserveSightingRequest>,
) -> impl Responder {
    let id_str = path.into_inner();
    debug!(id = %id_str, "POST /sightings/{{id}}/observations");

    let id = match SightingId::new(&id_str) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    let observed_at = UNIX_EPOCH + Duration::from_secs(body.observed_at);

    let command = ObserveSightingCommand { id, observed_at };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<ObserveSightingResponse>()
                .expect("Unexpected response type from ObserveSightingCommandHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "NotFound" => {
                        warn!(id = %id_str, "Sighting not found for observation");
                        HttpResponse::NotFound().body(error.message.clone())
                    }
                    _ => {
                        warn!(id = %id_str, error = %error.message, "Failed to observe sighting");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else {
                info!(id = %id_str, "Sighting observed");
                HttpResponse::Ok().finish()
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
