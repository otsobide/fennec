//! GET /ipv6-addresses/{id} handler for the CTI API.

use actix_web::{get, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::ipv6_address::application::find_ipv6_address::find_ipv6_address_query::FindIpv6AddressQuery;
use kernel::ipv6_address::application::find_ipv6_address::find_ipv6_address_response::FindIpv6AddressResponse;
use kernel::ipv6_address::domain::value_objects::ipv6_address_id::Ipv6AddressId;

use crate::ipv6_address::response_dtos::ipv6_address_response::Ipv6AddressResponse;
use crate::AppState;

/// Handles `GET /ipv6-addresses/{id}`.
///
/// # Responses
///
/// - `200 OK` – the address, as an [`Ipv6AddressResponse`].
/// - `400 Bad Request` – the path id is not a UUID v5.
/// - `404 Not Found` – no address is registered under the id.
/// - `500 Internal Server Error` – unexpected error.
#[get("/ipv6-addresses/{id}")]
pub async fn handler(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    debug!(id = %id_str, "GET /ipv6-addresses/{{id}}");

    let id = match Ipv6AddressId::new(&id_str) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    find_and_respond(&state, id).await
}

/// Dispatches a [`FindIpv6AddressQuery`] and maps its response to HTTP.
///
/// Shared with the lookup by value, which derives the id first.
pub(crate) async fn find_and_respond(state: &AppState, id: Ipv6AddressId) -> HttpResponse {
    let query = FindIpv6AddressQuery { id };

    match state.query_bus.ask(Box::new(query)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<FindIpv6AddressResponse>()
                .expect("Unexpected response type from FindIpv6AddressQueryHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "NotFound" => {
                        info!(id = %id, "IPv6 address not found");
                        HttpResponse::NotFound().body(error.message.clone())
                    }
                    _ => {
                        warn!(id = %id, error = %error.message, "Failed to find IPv6 address");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else if let Some(ref entry) = response.ipv6_address {
                info!(id = %id, "IPv6 address found");
                HttpResponse::Ok().json(Ipv6AddressResponse::from_entry(entry))
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(e) => {
            warn!(id = %id, error = %e, "Failed to find IPv6 address");
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}
