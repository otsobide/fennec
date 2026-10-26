//! DELETE /ipv4-networks/{id} handler for the CTI API.

use actix_web::{delete, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::ipv4_network::application::delete_ipv4_network::delete_ipv4_network_command::DeleteIpv4NetworkCommand;
use kernel::ipv4_network::application::delete_ipv4_network::delete_ipv4_network_response::DeleteIpv4NetworkResponse;
use kernel::ipv4_network::domain::value_objects::ipv4_network_id::Ipv4NetworkId;

use crate::AppState;

/// Handles `DELETE /ipv4-networks/{id}`.
///
/// # Responses
///
/// - `204 No Content` – the network was deleted.
/// - `400 Bad Request` – the path id is not a UUID v5.
/// - `404 Not Found` – no network is registered under the id.
/// - `500 Internal Server Error` – unexpected error.
#[delete("/ipv4-networks/{id}")]
pub async fn handler(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    debug!(id = %id_str, "DELETE /ipv4-networks/{{id}}");

    let id = match Ipv4NetworkId::new(&id_str) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    let command = DeleteIpv4NetworkCommand { id };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<DeleteIpv4NetworkResponse>()
                .expect("Unexpected response type from DeleteIpv4NetworkCommandHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "NotFound" => {
                        warn!(id = %id_str, "IPv4 network not found for deletion");
                        HttpResponse::NotFound().body(error.message.clone())
                    }
                    _ => {
                        warn!(id = %id_str, error = %error.message, "Failed to delete IPv4 network");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else {
                info!(id = %id_str, "IPv4 network deleted");
                HttpResponse::NoContent().finish()
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
