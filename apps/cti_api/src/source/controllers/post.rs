//! POST /sources handler for the CTI API.

use actix_web::{post, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::source::application::create_source::create_source_command::CreateSourceCommand;
use kernel::source::application::create_source::create_source_response::CreateSourceResponse;
use kernel::source::domain::value_objects::source_description::SourceDescription;
use kernel::source::domain::value_objects::source_id::SourceId;
use kernel::source::domain::value_objects::source_status::SourceStatus;
use kernel::source::domain::value_objects::source_type::SourceType;

use crate::source::request_dtos::create_source_request::CreateSourceRequest;
use crate::AppState;

/// Handles `POST /sources`.
///
/// # Responses
///
/// - `201 Created` – the source was persisted successfully.
/// - `400 Bad Request` – the payload contained an invalid id / source type / status.
/// - `409 Conflict` – a source with the given id already exists.
/// - `500 Internal Server Error` – unexpected error.
#[post("/sources")]
pub async fn handler(
    state: web::Data<AppState>,
    body: web::Json<CreateSourceRequest>,
) -> impl Responder {
    debug!(id = %body.id, "POST /sources");

    let id = match SourceId::new(&body.id) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    let source_type = match SourceType::from_str(&body.source_type) {
        Ok(t) => t,
        Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
    };

    let status = match SourceStatus::from_str(&body.status) {
        Ok(s) => s,
        Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
    };

    let description = match SourceDescription::new(body.description.clone()) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    let command = CreateSourceCommand {
        id,
        source_type,
        status,
        description,
    };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<CreateSourceResponse>()
                .expect("Unexpected response type from CreateSourceCommandHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "AlreadyExists" => {
                        warn!(id = %body.id, "Source already exists");
                        HttpResponse::Conflict().body(error.message.clone())
                    }
                    "NotFound" => HttpResponse::NotFound().body(error.message.clone()),
                    _ => {
                        warn!(id = %body.id, error = %error.message, "Failed to create source");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else {
                info!(id = %body.id, "Source created");
                HttpResponse::Created().finish()
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
