//! DELETE /dns-resolutions/{id}/observations/{source_id} handler for the CTI
//! API.

use actix_web::{delete, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::dns_resolution::application::retract_dns_resolution::retract_dns_resolution_command::RetractDnsResolutionCommand;
use kernel::dns_resolution::application::retract_dns_resolution::retract_dns_resolution_response::RetractDnsResolutionResponse;
use kernel::dns_resolution::domain::value_objects::dns_resolution_id::DnsResolutionId;
use kernel::dns_resolution::domain::value_objects::dns_resolution_source_id::DnsResolutionSourceId;

use crate::AppState;

/// Handles `DELETE /dns-resolutions/{id}/observations/{source_id}`:
/// retracts the observation one source made of a resolution. Retracting
/// the last observation deletes the resolution.
///
/// # Responses
///
/// - `204 No Content` – the observation was retracted.
/// - `400 Bad Request` – the id is not a UUID v5 or the source id is not a
///   UUID v4.
/// - `404 Not Found` – no resolution is recorded under the id, or the source
///   has not observed it.
/// - `500 Internal Server Error` – unexpected error.
#[delete("/dns-resolutions/{id}/observations/{source_id}")]
pub async fn handler(
    state: web::Data<AppState>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let (id_str, source_id_str) = path.into_inner();
    debug!(
        id = %id_str,
        source_id = %source_id_str,
        "DELETE /dns-resolutions/{{id}}/observations/{{source_id}}"
    );

    let id = match DnsResolutionId::new(&id_str) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };
    let source_id = match DnsResolutionSourceId::new(&source_id_str) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    let command = RetractDnsResolutionCommand { id, source_id };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<RetractDnsResolutionResponse>()
                .expect("Unexpected response type from RetractDnsResolutionCommandHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "NotFound" => {
                        warn!(id = %id_str, source_id = %source_id_str, "DNS resolution observation not found for retraction");
                        HttpResponse::NotFound().body(error.message.clone())
                    }
                    _ => {
                        warn!(id = %id_str, source_id = %source_id_str, error = %error.message, "Failed to retract DNS resolution observation");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else {
                info!(id = %id_str, source_id = %source_id_str, "DNS resolution observation retracted");
                HttpResponse::NoContent().finish()
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
