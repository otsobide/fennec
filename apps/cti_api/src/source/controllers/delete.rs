//! DELETE /sources/{id} handler for the CTI API.

use actix_web::{delete, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::source::application::delete_source::delete_source_command::DeleteSourceCommand;
use kernel::source::application::delete_source::delete_source_response::DeleteSourceResponse;
use kernel::source::domain::value_objects::source_id::SourceId;

use crate::AppState;

/// Handles `DELETE /sources/{id}`.
///
/// # Responses
///
/// - `204 No Content` – the source was deleted successfully.
/// - `400 Bad Request` – the path id is not a valid UUID.
/// - `404 Not Found` – no source exists for the given id.
/// - `500 Internal Server Error` – unexpected error.
#[delete("/sources/{id}")]
pub async fn handler(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    debug!(id = %id_str, "DELETE /sources/{{id}}");

    let id = match SourceId::new(&id_str) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    let command = DeleteSourceCommand { id };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<DeleteSourceResponse>()
                .expect("Unexpected response type from DeleteSourceCommandHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "NotFound" => {
                        warn!(id = %id_str, "Source not found for deletion");
                        HttpResponse::NotFound().body(error.message.clone())
                    }
                    _ => {
                        warn!(id = %id_str, error = %error.message, "Failed to delete source");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else {
                info!(id = %id_str, "Source deleted");
                HttpResponse::NoContent().finish()
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
