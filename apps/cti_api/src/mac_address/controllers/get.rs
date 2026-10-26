//! GET /mac-addresses/{id} handler for the CTI API.

use actix_web::{get, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::mac_address::application::find_mac_address::find_mac_address_query::FindMacAddressQuery;
use kernel::mac_address::application::find_mac_address::find_mac_address_response::FindMacAddressResponse;
use kernel::mac_address::domain::value_objects::mac_address_id::MacAddressId;

use crate::mac_address::response_dtos::mac_address_response::MacAddressResponse;
use crate::AppState;

/// Handles `GET /mac-addresses/{id}`.
///
/// # Responses
///
/// - `200 OK` – the address, as a [`MacAddressResponse`].
/// - `400 Bad Request` – the path id is not a UUID v5.
/// - `404 Not Found` – no address is registered under the id.
/// - `500 Internal Server Error` – unexpected error.
#[get("/mac-addresses/{id}")]
pub async fn handler(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    debug!(id = %id_str, "GET /mac-addresses/{{id}}");

    let id = match MacAddressId::new(&id_str) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    find_and_respond(&state, id).await
}

/// Dispatches a [`FindMacAddressQuery`] and maps its response to HTTP.
///
/// Shared with the lookup by value, which derives the id first.
pub(crate) async fn find_and_respond(state: &AppState, id: MacAddressId) -> HttpResponse {
    let query = FindMacAddressQuery { id };

    match state.query_bus.ask(Box::new(query)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<FindMacAddressResponse>()
                .expect("Unexpected response type from FindMacAddressQueryHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "NotFound" => {
                        info!(id = %id, "MAC address not found");
                        HttpResponse::NotFound().body(error.message.clone())
                    }
                    _ => {
                        warn!(id = %id, error = %error.message, "Failed to find MAC address");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else if let Some(ref entry) = response.mac_address {
                info!(id = %id, "MAC address found");
                HttpResponse::Ok().json(MacAddressResponse::from_entry(entry))
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(e) => {
            warn!(id = %id, error = %e, "Failed to find MAC address");
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}
