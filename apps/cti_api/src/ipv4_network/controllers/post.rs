//! POST /ipv4-networks handler for the CTI API.

use actix_web::{post, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::ipv4_network::application::create_ipv4_network::create_ipv4_network_command::CreateIpv4NetworkCommand;
use kernel::ipv4_network::application::create_ipv4_network::create_ipv4_network_response::CreateIpv4NetworkResponse;
use kernel::ipv4_network::domain::value_objects::ipv4_network_id::Ipv4NetworkId;
use kernel::ipv4_network::domain::value_objects::ipv4_network_value::Ipv4NetworkValue;

use crate::ipv4_network::request_dtos::create_ipv4_network_request::CreateIpv4NetworkRequest;
use crate::ipv4_network::response_dtos::ipv4_network_reference_response::Ipv4NetworkReferenceResponse;
use crate::AppState;

/// Handles `POST /ipv4-networks`.
///
/// # Responses
///
/// - `201 Created` – the network was registered; `Location` points at it and
///   the body carries its `id` and `stix_id`.
/// - `400 Bad Request` – the value is not a valid IPv4 network, or the
///   body has fields other than `value`.
/// - `409 Conflict` – the network is already registered; the body carries its
///   `id` and `stix_id`.
/// - `500 Internal Server Error` – unexpected error.
#[post("/ipv4-networks")]
pub async fn handler(
    state: web::Data<AppState>,
    body: web::Json<CreateIpv4NetworkRequest>,
) -> impl Responder {
    debug!(value = %body.value, "POST /ipv4-networks");

    let value = match Ipv4NetworkValue::new(&body.value) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };
    let reference = Ipv4NetworkReferenceResponse::from_id(&Ipv4NetworkId::from_value(&value));

    let command = CreateIpv4NetworkCommand { value };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<CreateIpv4NetworkResponse>()
                .expect("Unexpected response type from CreateIpv4NetworkCommandHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "AlreadyExists" => {
                        warn!(id = %reference.id, "IPv4 network already exists");
                        HttpResponse::Conflict().json(reference)
                    }
                    _ => {
                        warn!(id = %reference.id, error = %error.message, "Failed to create IPv4 network");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else {
                info!(id = %reference.id, "IPv4 network created");
                HttpResponse::Created()
                    .insert_header(("Location", format!("/ipv4-networks/{}", reference.id)))
                    .json(reference)
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
