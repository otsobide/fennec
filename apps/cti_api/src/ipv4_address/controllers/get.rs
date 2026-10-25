//! GET /ipv4-addresses/{id} handler for the CTI API.

use actix_web::{get, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::ipv4_address::application::find_ipv4_address::find_ipv4_address_query::FindIpv4AddressQuery;
use kernel::ipv4_address::application::find_ipv4_address::find_ipv4_address_response::FindIpv4AddressResponse;
use kernel::ipv4_address::domain::value_objects::ipv4_address_id::Ipv4AddressId;

use crate::ipv4_address::response_dtos::ipv4_address_response::Ipv4AddressResponse;
use crate::AppState;

/// Handles `GET /ipv4-addresses/{id}`.
///
/// # Responses
///
/// - `200 OK` – the address, as an [`Ipv4AddressResponse`].
/// - `400 Bad Request` – the path id is not a UUID v5.
/// - `404 Not Found` – no address is registered under the id.
/// - `500 Internal Server Error` – unexpected error.
#[get("/ipv4-addresses/{id}")]
pub async fn handler(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    debug!(id = %id_str, "GET /ipv4-addresses/{{id}}");

    let id = match Ipv4AddressId::new(&id_str) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    find_and_respond(&state, id).await
}

/// Dispatches a [`FindIpv4AddressQuery`] and maps its response to HTTP.
///
/// Shared with the lookup by value, which derives the id first.
pub(crate) async fn find_and_respond(state: &AppState, id: Ipv4AddressId) -> HttpResponse {
    let query = FindIpv4AddressQuery { id };

    match state.query_bus.ask(Box::new(query)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<FindIpv4AddressResponse>()
                .expect("Unexpected response type from FindIpv4AddressQueryHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "NotFound" => {
                        info!(id = %id, "Ipv4 address not found");
                        HttpResponse::NotFound().body(error.message.clone())
                    }
                    _ => {
                        warn!(id = %id, error = %error.message, "Failed to find ipv4 address");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else if let Some(ref entry) = response.ipv4_address {
                info!(id = %id, "Ipv4 address found");
                HttpResponse::Ok().json(Ipv4AddressResponse::from_entry(entry))
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(e) => {
            warn!(id = %id, error = %e, "Failed to find ipv4 address");
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}
