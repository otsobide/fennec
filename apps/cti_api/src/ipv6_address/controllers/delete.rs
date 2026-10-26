//! DELETE /ipv6-addresses/{id} handler for the CTI API.

use actix_web::{delete, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::ipv6_address::application::delete_ipv6_address::delete_ipv6_address_command::DeleteIpv6AddressCommand;
use kernel::ipv6_address::application::delete_ipv6_address::delete_ipv6_address_response::DeleteIpv6AddressResponse;
use kernel::ipv6_address::domain::value_objects::ipv6_address_id::Ipv6AddressId;

use crate::AppState;

/// Handles `DELETE /ipv6-addresses/{id}`.
///
/// # Responses
///
/// - `204 No Content` – the address was deleted.
/// - `400 Bad Request` – the path id is not a UUID v5.
/// - `404 Not Found` – no address is registered under the id.
/// - `500 Internal Server Error` – unexpected error.
#[delete("/ipv6-addresses/{id}")]
pub async fn handler(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    debug!(id = %id_str, "DELETE /ipv6-addresses/{{id}}");

    let id = match Ipv6AddressId::new(&id_str) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    let command = DeleteIpv6AddressCommand { id };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<DeleteIpv6AddressResponse>()
                .expect("Unexpected response type from DeleteIpv6AddressCommandHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "NotFound" => {
                        warn!(id = %id_str, "IPv6 address not found for deletion");
                        HttpResponse::NotFound().body(error.message.clone())
                    }
                    _ => {
                        warn!(id = %id_str, error = %error.message, "Failed to delete IPv6 address");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else {
                info!(id = %id_str, "IPv6 address deleted");
                HttpResponse::NoContent().finish()
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
