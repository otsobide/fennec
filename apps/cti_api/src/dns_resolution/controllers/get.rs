//! GET /dns-resolutions/{id} handler for the CTI API.

use actix_web::{get, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::dns_resolution::application::find_dns_resolution::find_dns_resolution_query::FindDnsResolutionQuery;
use kernel::dns_resolution::application::find_dns_resolution::find_dns_resolution_response::FindDnsResolutionResponse;
use kernel::dns_resolution::domain::value_objects::dns_resolution_id::DnsResolutionId;

use crate::dns_resolution::response_dtos::dns_resolution_response::DnsResolutionResponse;
use crate::AppState;

/// Handles `GET /dns-resolutions/{id}`.
///
/// # Responses
///
/// - `200 OK` – the resolution with every source's observation, as a
///   [`DnsResolutionResponse`].
/// - `400 Bad Request` – the path id is not a UUID v5.
/// - `404 Not Found` – no resolution is recorded under the id.
/// - `500 Internal Server Error` – unexpected error.
#[get("/dns-resolutions/{id}")]
pub async fn handler(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    debug!(id = %id_str, "GET /dns-resolutions/{{id}}");

    let id = match DnsResolutionId::new(&id_str) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    let query = FindDnsResolutionQuery { id };

    match state.query_bus.ask(Box::new(query)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<FindDnsResolutionResponse>()
                .expect("Unexpected response type from FindDnsResolutionQueryHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "NotFound" => {
                        info!(id = %id_str, "DNS resolution not found");
                        HttpResponse::NotFound().body(error.message.clone())
                    }
                    _ => {
                        warn!(id = %id_str, error = %error.message, "Failed to find DNS resolution");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else if let Some(ref entry) = response.dns_resolution {
                info!(id = %id_str, "DNS resolution found");
                HttpResponse::Ok().json(DnsResolutionResponse::from_entry(entry))
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(e) => {
            warn!(id = %id_str, error = %e, "Failed to find DNS resolution");
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}
