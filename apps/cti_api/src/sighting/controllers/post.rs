//! POST /sightings handler for the CTI API.

use std::time::{Duration, UNIX_EPOCH};

use actix_web::{post, web, HttpResponse, Responder};
use serde::Serialize;
use tracing::{debug, info, warn};

use kernel::sighting::application::create_sighting::create_sighting_command::CreateSightingCommand;
use kernel::sighting::application::create_sighting::create_sighting_response::CreateSightingResponse;
use kernel::sighting::domain::value_objects::sighting_id::SightingId;
use kernel::sighting::domain::value_objects::sighting_ioc_id::SightingIocId;
use kernel::sighting::domain::value_objects::sighting_source_id::SightingSourceId;

use crate::sighting::request_dtos::create_sighting_request::CreateSightingRequest;
use crate::AppState;

#[derive(Serialize)]
struct PairAlreadyExistsBody {
    message: String,
    existing_id: String,
}

#[post("/sightings")]
pub async fn handler(
    state: web::Data<AppState>,
    body: web::Json<CreateSightingRequest>,
) -> impl Responder {
    debug!(id = %body.id, "POST /sightings");

    let id = match SightingId::new(&body.id) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    let ioc_id = match SightingIocId::new(&body.ioc_id) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    let source_id = match SightingSourceId::new(&body.source_id) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    let observed_at = UNIX_EPOCH + Duration::from_secs(body.observed_at);

    let command = CreateSightingCommand {
        id,
        ioc_id,
        source_id,
        observed_at,
    };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<CreateSightingResponse>()
                .expect("Unexpected response type from CreateSightingCommandHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "AlreadyExists" => {
                        warn!(id = %body.id, "Sighting already exists");
                        HttpResponse::Conflict().body(error.message.clone())
                    }
                    "PairAlreadyExists" => {
                        let existing_id = error.existing_id.clone().unwrap_or_default();
                        warn!(id = %body.id, existing_id = %existing_id, "Sighting pair already exists");
                        HttpResponse::Conflict().json(PairAlreadyExistsBody {
                            message: error.message.clone(),
                            existing_id,
                        })
                    }
                    "NotFound" => HttpResponse::NotFound().body(error.message.clone()),
                    _ => {
                        warn!(id = %body.id, error = %error.message, "Failed to create sighting");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else {
                info!(id = %body.id, "Sighting created");
                HttpResponse::Created().finish()
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
