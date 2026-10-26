//! GET /ipv4-networks/{id} handler for the CTI API.

use actix_web::{get, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::ipv4_network::application::find_ipv4_network::find_ipv4_network_query::FindIpv4NetworkQuery;
use kernel::ipv4_network::application::find_ipv4_network::find_ipv4_network_response::FindIpv4NetworkResponse;
use kernel::ipv4_network::domain::value_objects::ipv4_network_id::Ipv4NetworkId;

use crate::ipv4_network::response_dtos::ipv4_network_response::Ipv4NetworkResponse;
use crate::AppState;

/// Handles `GET /ipv4-networks/{id}`.
///
/// # Responses
///
/// - `200 OK` – the network, as an [`Ipv4NetworkResponse`].
/// - `400 Bad Request` – the path id is not a UUID v5.
/// - `404 Not Found` – no network is registered under the id.
/// - `500 Internal Server Error` – unexpected error.
#[get("/ipv4-networks/{id}")]
pub async fn handler(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    debug!(id = %id_str, "GET /ipv4-networks/{{id}}");

    let id = match Ipv4NetworkId::new(&id_str) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    find_and_respond(&state, id).await
}

/// Dispatches a [`FindIpv4NetworkQuery`] and maps its response to HTTP.
///
/// Shared with the lookup by value, which derives the id first.
pub(crate) async fn find_and_respond(state: &AppState, id: Ipv4NetworkId) -> HttpResponse {
    let query = FindIpv4NetworkQuery { id };

    match state.query_bus.ask(Box::new(query)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<FindIpv4NetworkResponse>()
                .expect("Unexpected response type from FindIpv4NetworkQueryHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "NotFound" => {
                        info!(id = %id, "IPv4 network not found");
                        HttpResponse::NotFound().body(error.message.clone())
                    }
                    _ => {
                        warn!(id = %id, error = %error.message, "Failed to find IPv4 network");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else if let Some(ref entry) = response.ipv4_network {
                info!(id = %id, "IPv4 network found");
                HttpResponse::Ok().json(Ipv4NetworkResponse::from_entry(entry))
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(e) => {
            warn!(id = %id, error = %e, "Failed to find IPv4 network");
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}
