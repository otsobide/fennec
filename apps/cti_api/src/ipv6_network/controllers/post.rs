//! POST /ipv6-networks handler for the CTI API.

use actix_web::{post, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::ipv6_network::application::create_ipv6_network::create_ipv6_network_command::CreateIpv6NetworkCommand;
use kernel::ipv6_network::application::create_ipv6_network::create_ipv6_network_response::CreateIpv6NetworkResponse;
use kernel::ipv6_network::domain::value_objects::ipv6_network_id::Ipv6NetworkId;
use kernel::ipv6_network::domain::value_objects::ipv6_network_value::Ipv6NetworkValue;

use crate::ipv6_network::request_dtos::create_ipv6_network_request::CreateIpv6NetworkRequest;
use crate::ipv6_network::response_dtos::ipv6_network_reference_response::Ipv6NetworkReferenceResponse;
use crate::AppState;

/// Handles `POST /ipv6-networks`.
///
/// # Responses
///
/// - `201 Created` – the network was registered; `Location` points at it and
///   the body carries its `id` and `stix_id`.
/// - `400 Bad Request` – the value is not a valid IPv6 network, or the
///   body has fields other than `value`.
/// - `409 Conflict` – the network is already registered; the body carries its
///   `id` and `stix_id`.
/// - `500 Internal Server Error` – unexpected error.
#[post("/ipv6-networks")]
pub async fn handler(
    state: web::Data<AppState>,
    body: web::Json<CreateIpv6NetworkRequest>,
) -> impl Responder {
    debug!(value = %body.value, "POST /ipv6-networks");

    let value = match Ipv6NetworkValue::new(&body.value) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };
    let reference = Ipv6NetworkReferenceResponse::from_id(&Ipv6NetworkId::from_value(&value));

    let command = CreateIpv6NetworkCommand { value };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<CreateIpv6NetworkResponse>()
                .expect("Unexpected response type from CreateIpv6NetworkCommandHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "AlreadyExists" => {
                        warn!(id = %reference.id, "IPv6 network already exists");
                        HttpResponse::Conflict().json(reference)
                    }
                    _ => {
                        warn!(id = %reference.id, error = %error.message, "Failed to create IPv6 network");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else {
                info!(id = %reference.id, "IPv6 network created");
                HttpResponse::Created()
                    .insert_header(("Location", format!("/ipv6-networks/{}", reference.id)))
                    .json(reference)
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
