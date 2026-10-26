//! GET /ipv6-networks/{id} handler for the CTI API.

use actix_web::{get, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::ipv6_network::application::find_ipv6_network::find_ipv6_network_query::FindIpv6NetworkQuery;
use kernel::ipv6_network::application::find_ipv6_network::find_ipv6_network_response::FindIpv6NetworkResponse;
use kernel::ipv6_network::domain::value_objects::ipv6_network_id::Ipv6NetworkId;

use crate::ipv6_network::response_dtos::ipv6_network_response::Ipv6NetworkResponse;
use crate::AppState;

/// Handles `GET /ipv6-networks/{id}`.
///
/// # Responses
///
/// - `200 OK` – the network, as an [`Ipv6NetworkResponse`].
/// - `400 Bad Request` – the path id is not a UUID v5.
/// - `404 Not Found` – no network is registered under the id.
/// - `500 Internal Server Error` – unexpected error.
#[get("/ipv6-networks/{id}")]
pub async fn handler(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    debug!(id = %id_str, "GET /ipv6-networks/{{id}}");

    let id = match Ipv6NetworkId::new(&id_str) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    find_and_respond(&state, id).await
}

/// Dispatches a [`FindIpv6NetworkQuery`] and maps its response to HTTP.
///
/// Shared with the lookup by value, which derives the id first.
pub(crate) async fn find_and_respond(state: &AppState, id: Ipv6NetworkId) -> HttpResponse {
    let query = FindIpv6NetworkQuery { id };

    match state.query_bus.ask(Box::new(query)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<FindIpv6NetworkResponse>()
                .expect("Unexpected response type from FindIpv6NetworkQueryHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "NotFound" => {
                        info!(id = %id, "IPv6 network not found");
                        HttpResponse::NotFound().body(error.message.clone())
                    }
                    _ => {
                        warn!(id = %id, error = %error.message, "Failed to find IPv6 network");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else if let Some(ref entry) = response.ipv6_network {
                info!(id = %id, "IPv6 network found");
                HttpResponse::Ok().json(Ipv6NetworkResponse::from_entry(entry))
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(e) => {
            warn!(id = %id, error = %e, "Failed to find IPv6 network");
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}
