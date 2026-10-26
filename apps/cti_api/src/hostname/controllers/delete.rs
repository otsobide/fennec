//! DELETE /hostnames/{id} handler for the CTI API.

use actix_web::{delete, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::hostname::application::delete_hostname::delete_hostname_command::DeleteHostnameCommand;
use kernel::hostname::application::delete_hostname::delete_hostname_response::DeleteHostnameResponse;
use kernel::hostname::domain::value_objects::hostname_id::HostnameId;

use crate::AppState;

/// Handles `DELETE /hostnames/{id}`.
///
/// # Responses
///
/// - `204 No Content` – the hostname was deleted.
/// - `400 Bad Request` – the path id is not a UUID v5.
/// - `404 Not Found` – no hostname is registered under the id.
/// - `500 Internal Server Error` – unexpected error.
#[delete("/hostnames/{id}")]
pub async fn handler(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    debug!(id = %id_str, "DELETE /hostnames/{{id}}");

    let id = match HostnameId::new(&id_str) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    let command = DeleteHostnameCommand { id };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<DeleteHostnameResponse>()
                .expect("Unexpected response type from DeleteHostnameCommandHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "NotFound" => {
                        warn!(id = %id_str, "Hostname not found for deletion");
                        HttpResponse::NotFound().body(error.message.clone())
                    }
                    _ => {
                        warn!(id = %id_str, error = %error.message, "Failed to delete hostname");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else {
                info!(id = %id_str, "Hostname deleted");
                HttpResponse::NoContent().finish()
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
