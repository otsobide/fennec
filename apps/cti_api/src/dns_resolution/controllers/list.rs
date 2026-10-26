//! GET /dns-resolutions?domain_name=|ipv4_address=|ipv6_address= handler for
//! the CTI API.

use actix_web::{get, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::dns_resolution::application::list_dns_resolutions_by_address::list_dns_resolutions_by_address_query::ListDnsResolutionsByAddressQuery;
use kernel::dns_resolution::application::list_dns_resolutions_by_address::list_dns_resolutions_by_address_response::ListDnsResolutionsByAddressResponse;
use kernel::dns_resolution::application::list_dns_resolutions_by_domain_name::list_dns_resolutions_by_domain_name_query::ListDnsResolutionsByDomainNameQuery;
use kernel::dns_resolution::application::list_dns_resolutions_by_domain_name::list_dns_resolutions_by_domain_name_response::ListDnsResolutionsByDomainNameResponse;
use kernel::dns_resolution::domain::value_objects::dns_resolution_answer::DnsResolutionAnswer;
use kernel::dns_resolution::domain::value_objects::dns_resolution_subject::DnsResolutionSubject;

use crate::dns_resolution::request_dtos::list_dns_resolutions_request::ListDnsResolutionsRequest;
use crate::dns_resolution::response_dtos::dns_resolution_list_response::DnsResolutionListResponse;
use crate::AppState;

/// Handles `GET /dns-resolutions?domain_name=<name>`,
/// `?ipv4_address=<address>` and `?ipv6_address=<address>`: the resolutions
/// of a domain name, ordered by the address of their answer with IPv4
/// before IPv6, or the resolutions to an address, ordered by domain name.
///
/// The parameter names the kind of the endpoint, which is never inferred
/// from the value. At most [`MAX_LISTED`] resolutions are listed;
/// `truncated` says whether more are recorded.
///
/// # Responses
///
/// - `200 OK` – the resolutions, possibly none, as a
///   [`DnsResolutionListResponse`].
/// - `400 Bad Request` – not exactly one parameter is given, a parameter is
///   unknown, or the value is not valid for its kind.
/// - `500 Internal Server Error` – unexpected error.
///
/// [`MAX_LISTED`]: kernel::dns_resolution::application::list_dns_resolutions_by_domain_name::dns_resolutions_by_domain_name_lister::MAX_LISTED
#[get("/dns-resolutions")]
pub async fn handler(
    state: web::Data<AppState>,
    query: web::Query<ListDnsResolutionsRequest>,
) -> impl Responder {
    let query = query.into_inner();
    debug!(
        domain_name = ?query.domain_name,
        ipv4_address = ?query.ipv4_address,
        ipv6_address = ?query.ipv6_address,
        "GET /dns-resolutions"
    );

    match (query.domain_name, query.ipv4_address, query.ipv6_address) {
        (Some(name), None, None) => match DnsResolutionSubject::domain_name(&name) {
            Ok(subject) => list_by_domain_name(&state, subject).await,
            Err(error) => HttpResponse::BadRequest().body(error.to_string()),
        },
        (None, Some(address), None) => match DnsResolutionAnswer::ipv4_address(&address) {
            Ok(answer) => list_by_address(&state, answer).await,
            Err(error) => HttpResponse::BadRequest().body(error.to_string()),
        },
        (None, None, Some(address)) => match DnsResolutionAnswer::ipv6_address(&address) {
            Ok(answer) => list_by_address(&state, answer).await,
            Err(error) => HttpResponse::BadRequest().body(error.to_string()),
        },
        _ => HttpResponse::BadRequest()
            .body("exactly one of domain_name, ipv4_address or ipv6_address is required"),
    }
}

/// Asks a [`ListDnsResolutionsByDomainNameQuery`] and maps its response to
/// HTTP.
async fn list_by_domain_name(state: &AppState, subject: DnsResolutionSubject) -> HttpResponse {
    let subject_str = subject.to_string();
    let query = ListDnsResolutionsByDomainNameQuery { subject };

    match state.query_bus.ask(Box::new(query)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<ListDnsResolutionsByDomainNameResponse>()
                .expect("Unexpected response type from ListDnsResolutionsByDomainNameQueryHandler");

            if let Some(ref error) = response.error {
                warn!(domain_name = %subject_str, error = %error.message, "Failed to list DNS resolutions by domain name");
                return HttpResponse::InternalServerError().body(error.message.clone());
            }

            info!(domain_name = %subject_str, count = response.dns_resolutions.len(), truncated = response.truncated, "DNS resolutions listed by domain name");
            HttpResponse::Ok().json(DnsResolutionListResponse::from_entries(
                &response.dns_resolutions,
                response.truncated,
            ))
        }
        Err(e) => {
            warn!(domain_name = %subject_str, error = %e, "Failed to list DNS resolutions by domain name");
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

/// Asks a [`ListDnsResolutionsByAddressQuery`] and maps its response to
/// HTTP.
async fn list_by_address(state: &AppState, answer: DnsResolutionAnswer) -> HttpResponse {
    let query = ListDnsResolutionsByAddressQuery { answer };

    match state.query_bus.ask(Box::new(query)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<ListDnsResolutionsByAddressResponse>()
                .expect("Unexpected response type from ListDnsResolutionsByAddressQueryHandler");

            if let Some(ref error) = response.error {
                warn!(answer = %answer, error = %error.message, "Failed to list DNS resolutions by address");
                return HttpResponse::InternalServerError().body(error.message.clone());
            }

            info!(answer = %answer, count = response.dns_resolutions.len(), truncated = response.truncated, "DNS resolutions listed by address");
            HttpResponse::Ok().json(DnsResolutionListResponse::from_entries(
                &response.dns_resolutions,
                response.truncated,
            ))
        }
        Err(e) => {
            warn!(answer = %answer, error = %e, "Failed to list DNS resolutions by address");
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}
