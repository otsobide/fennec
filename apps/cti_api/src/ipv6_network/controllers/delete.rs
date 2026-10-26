//! DELETE /ipv6-networks/{id} handler for the CTI API.

use actix_web::{delete, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::ipv6_network::application::delete_ipv6_network::delete_ipv6_network_command::DeleteIpv6NetworkCommand;
use kernel::ipv6_network::application::delete_ipv6_network::delete_ipv6_network_response::DeleteIpv6NetworkResponse;
use kernel::ipv6_network::domain::value_objects::ipv6_network_id::Ipv6NetworkId;

use crate::AppState;

/// Handles `DELETE /ipv6-networks/{id}`.
///
/// # Responses
///
/// - `204 No Content` – the network was deleted.
/// - `400 Bad Request` – the path id is not a UUID v5.
/// - `404 Not Found` – no network is registered under the id.
/// - `500 Internal Server Error` – unexpected error.
#[delete("/ipv6-networks/{id}")]
pub async fn handler(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    debug!(id = %id_str, "DELETE /ipv6-networks/{{id}}");

    let id = match Ipv6NetworkId::new(&id_str) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    let command = DeleteIpv6NetworkCommand { id };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<DeleteIpv6NetworkResponse>()
                .expect("Unexpected response type from DeleteIpv6NetworkCommandHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "NotFound" => {
                        warn!(id = %id_str, "IPv6 network not found for deletion");
                        HttpResponse::NotFound().body(error.message.clone())
                    }
                    _ => {
                        warn!(id = %id_str, error = %error.message, "Failed to delete IPv6 network");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else {
                info!(id = %id_str, "IPv6 network deleted");
                HttpResponse::NoContent().finish()
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
