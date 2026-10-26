//! GET /dns-resolutions/co-resolved?domain_name= handler for the CTI API.

use actix_web::{get, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::dns_resolution::application::list_co_resolved_domain_names::list_co_resolved_domain_names_query::ListCoResolvedDomainNamesQuery;
use kernel::dns_resolution::application::list_co_resolved_domain_names::list_co_resolved_domain_names_response::ListCoResolvedDomainNamesResponse;
use kernel::dns_resolution::domain::value_objects::dns_resolution_subject::DnsResolutionSubject;

use crate::dns_resolution::request_dtos::list_co_resolved_domain_names_request::ListCoResolvedDomainNamesRequest;
use crate::dns_resolution::response_dtos::co_resolved_domain_names_response::CoResolvedDomainNamesResponse;
use crate::AppState;

/// Handles `GET /dns-resolutions/co-resolved?domain_name=<name>`: the
/// domain names that resolved to one of the addresses the name resolved
/// to, at overlapping times.
///
/// It must be registered before `GET /dns-resolutions/{id}`, which would
/// otherwise take `co-resolved` for an id.
///
/// # Responses
///
/// - `200 OK` – the co-resolved names, possibly none, as a
///   [`CoResolvedDomainNamesResponse`].
/// - `400 Bad Request` – `domain_name` is missing or invalid, or a
///   parameter is unknown.
/// - `500 Internal Server Error` – unexpected error.
#[get("/dns-resolutions/co-resolved")]
pub async fn handler(
    state: web::Data<AppState>,
    query: web::Query<ListCoResolvedDomainNamesRequest>,
) -> impl Responder {
    debug!(domain_name = %query.domain_name, "GET /dns-resolutions/co-resolved");

    let subject = match DnsResolutionSubject::domain_name(&query.domain_name) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    let query = ListCoResolvedDomainNamesQuery { subject };

    match state.query_bus.ask(Box::new(query)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<ListCoResolvedDomainNamesResponse>()
                .expect("Unexpected response type from ListCoResolvedDomainNamesQueryHandler");

            if let Some(ref error) = response.error {
                warn!(error = %error.message, "Failed to list co-resolved domain names");
                return HttpResponse::InternalServerError().body(error.message.clone());
            }
            let Some(ref domain_name) = response.domain_name else {
                warn!("Co-resolved domain names listed without the queried domain name");
                return HttpResponse::InternalServerError().finish();
            };

            info!(
                domain_name = %domain_name.value,
                co_resolved = response.co_resolved.len(),
                truncated = response.truncated,
                "Co-resolved domain names listed"
            );
            HttpResponse::Ok().json(CoResolvedDomainNamesResponse::from_entries(
                domain_name,
                &response.co_resolved,
                response.truncated,
            ))
        }
        Err(e) => {
            warn!(error = %e, "Failed to list co-resolved domain names");
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}
