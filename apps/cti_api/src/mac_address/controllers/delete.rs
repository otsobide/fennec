//! DELETE /mac-addresses/{id} handler for the CTI API.

use actix_web::{delete, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::mac_address::application::delete_mac_address::delete_mac_address_command::DeleteMacAddressCommand;
use kernel::mac_address::application::delete_mac_address::delete_mac_address_response::DeleteMacAddressResponse;
use kernel::mac_address::domain::value_objects::mac_address_id::MacAddressId;

use crate::AppState;

/// Handles `DELETE /mac-addresses/{id}`.
///
/// # Responses
///
/// - `204 No Content` – the address was deleted.
/// - `400 Bad Request` – the path id is not a UUID v5.
/// - `404 Not Found` – no address is registered under the id.
/// - `500 Internal Server Error` – unexpected error.
#[delete("/mac-addresses/{id}")]
pub async fn handler(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    debug!(id = %id_str, "DELETE /mac-addresses/{{id}}");

    let id = match MacAddressId::new(&id_str) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    let command = DeleteMacAddressCommand { id };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<DeleteMacAddressResponse>()
                .expect("Unexpected response type from DeleteMacAddressCommandHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "NotFound" => {
                        warn!(id = %id_str, "MAC address not found for deletion");
                        HttpResponse::NotFound().body(error.message.clone())
                    }
                    _ => {
                        warn!(id = %id_str, error = %error.message, "Failed to delete MAC address");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else {
                info!(id = %id_str, "MAC address deleted");
                HttpResponse::NoContent().finish()
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
