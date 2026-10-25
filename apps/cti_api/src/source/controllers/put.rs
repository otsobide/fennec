//! PUT /sources/{id} handler for the CTI API.

use actix_web::{put, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::source::application::update_source::update_source_command::UpdateSourceCommand;
use kernel::source::application::update_source::update_source_response::UpdateSourceResponse;
use kernel::source::domain::value_objects::source_description::SourceDescription;
use kernel::source::domain::value_objects::source_id::SourceId;
use kernel::source::domain::value_objects::source_status::SourceStatus;

use crate::source::request_dtos::update_source_request::UpdateSourceRequest;
use crate::AppState;

/// Handles `PUT /sources/{id}`.
///
/// # Responses
///
/// - `200 OK` – the source was updated successfully.
/// - `400 Bad Request` – invalid UUID or invalid status.
/// - `404 Not Found` – no source exists for the given id.
/// - `500 Internal Server Error` – unexpected error.
#[put("/sources/{id}")]
pub async fn handler(
    state: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<UpdateSourceRequest>,
) -> impl Responder {
    let id_str = path.into_inner();
    debug!(id = %id_str, "PUT /sources/{{id}}");

    let id = match SourceId::new(&id_str) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    let status = match SourceStatus::from_str(&body.status) {
        Ok(s) => s,
        Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
    };

    let description = match SourceDescription::new(body.description.clone()) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    let command = UpdateSourceCommand {
        id,
        status,
        description,
    };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<UpdateSourceResponse>()
                .expect("Unexpected response type from UpdateSourceCommandHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "NotFound" => {
                        warn!(id = %id_str, "Source not found for update");
                        HttpResponse::NotFound().body(error.message.clone())
                    }
                    "AlreadyExists" => HttpResponse::Conflict().body(error.message.clone()),
                    _ => {
                        warn!(id = %id_str, error = %error.message, "Failed to update source");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else {
                info!(id = %id_str, "Source updated");
                HttpResponse::Ok().finish()
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
