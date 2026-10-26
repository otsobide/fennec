//! DELETE /dns-resolutions/{id} handler for the CTI API.

use actix_web::{delete, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::dns_resolution::application::delete_dns_resolution::delete_dns_resolution_command::DeleteDnsResolutionCommand;
use kernel::dns_resolution::application::delete_dns_resolution::delete_dns_resolution_response::DeleteDnsResolutionResponse;
use kernel::dns_resolution::domain::value_objects::dns_resolution_id::DnsResolutionId;

use crate::AppState;

/// Handles `DELETE /dns-resolutions/{id}`: deletes a resolution with every
/// source's observation of it.
///
/// # Responses
///
/// - `204 No Content` – the resolution was deleted.
/// - `400 Bad Request` – the path id is not a UUID v5.
/// - `404 Not Found` – no resolution is recorded under the id.
/// - `500 Internal Server Error` – unexpected error.
#[delete("/dns-resolutions/{id}")]
pub async fn handler(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    debug!(id = %id_str, "DELETE /dns-resolutions/{{id}}");

    let id = match DnsResolutionId::new(&id_str) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    let command = DeleteDnsResolutionCommand { id };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<DeleteDnsResolutionResponse>()
                .expect("Unexpected response type from DeleteDnsResolutionCommandHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "NotFound" => {
                        warn!(id = %id_str, "DNS resolution not found for deletion");
                        HttpResponse::NotFound().body(error.message.clone())
                    }
                    _ => {
                        warn!(id = %id_str, error = %error.message, "Failed to delete DNS resolution");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else {
                info!(id = %id_str, "DNS resolution deleted");
                HttpResponse::NoContent().finish()
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
