//! DELETE /ipv4-addresses/{id} handler for the CTI API.

use actix_web::{delete, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::ipv4_address::application::delete_ipv4_address::delete_ipv4_address_command::DeleteIpv4AddressCommand;
use kernel::ipv4_address::application::delete_ipv4_address::delete_ipv4_address_response::DeleteIpv4AddressResponse;
use kernel::ipv4_address::domain::value_objects::ipv4_address_id::Ipv4AddressId;

use crate::AppState;

/// Handles `DELETE /ipv4-addresses/{id}`.
///
/// # Responses
///
/// - `204 No Content` – the address was deleted.
/// - `400 Bad Request` – the path id is not a UUID v5.
/// - `404 Not Found` – no address is registered under the id.
/// - `500 Internal Server Error` – unexpected error.
#[delete("/ipv4-addresses/{id}")]
pub async fn handler(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    debug!(id = %id_str, "DELETE /ipv4-addresses/{{id}}");

    let id = match Ipv4AddressId::new(&id_str) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    let command = DeleteIpv4AddressCommand { id };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<DeleteIpv4AddressResponse>()
                .expect("Unexpected response type from DeleteIpv4AddressCommandHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "NotFound" => {
                        warn!(id = %id_str, "Ipv4 address not found for deletion");
                        HttpResponse::NotFound().body(error.message.clone())
                    }
                    _ => {
                        warn!(id = %id_str, error = %error.message, "Failed to delete ipv4 address");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else {
                info!(id = %id_str, "Ipv4 address deleted");
                HttpResponse::NoContent().finish()
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
