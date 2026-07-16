//! DELETE /sightings/{id} handler for the CTI API.

use actix_web::{delete, web, HttpResponse, Responder};
use tracing::{debug, info, warn};
use uuid::Uuid;

use ::sighting::sighting::application::delete_sighting::delete_sighting_command::DeleteSightingCommand;
use ::sighting::sighting::application::delete_sighting::delete_sighting_response::DeleteSightingResponse;
use ::sighting::sighting::domain::value_objects::sighting_id::SightingId;

use crate::AppState;

#[delete("/sightings/{id}")]
pub async fn handler(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let id_str = path.into_inner();
    debug!(id = %id_str, "DELETE /sightings/{{id}}");

    let id = match Uuid::parse_str(&id_str) {
        Ok(uuid) => SightingId::from_uuid(uuid),
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format"),
    };

    let command = DeleteSightingCommand { id };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<DeleteSightingResponse>()
                .expect("Unexpected response type from DeleteSightingCommandHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "NotFound" => {
                        warn!(id = %id_str, "Sighting not found for deletion");
                        HttpResponse::NotFound().body(error.message.clone())
                    }
                    _ => {
                        warn!(id = %id_str, error = %error.message, "Failed to delete sighting");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else {
                info!(id = %id_str, "Sighting deleted");
                HttpResponse::NoContent().finish()
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
