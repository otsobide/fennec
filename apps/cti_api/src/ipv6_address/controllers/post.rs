//! POST /ipv6-addresses handler for the CTI API.

use actix_web::{post, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::ipv6_address::application::create_ipv6_address::create_ipv6_address_command::CreateIpv6AddressCommand;
use kernel::ipv6_address::application::create_ipv6_address::create_ipv6_address_response::CreateIpv6AddressResponse;
use kernel::ipv6_address::domain::value_objects::ipv6_address_id::Ipv6AddressId;
use kernel::ipv6_address::domain::value_objects::ipv6_address_value::Ipv6AddressValue;

use crate::ipv6_address::request_dtos::create_ipv6_address_request::CreateIpv6AddressRequest;
use crate::ipv6_address::response_dtos::ipv6_address_reference_response::Ipv6AddressReferenceResponse;
use crate::AppState;

/// Handles `POST /ipv6-addresses`.
///
/// # Responses
///
/// - `201 Created` – the address was registered; `Location` points at it and
///   the body carries its `id` and `stix_id`.
/// - `400 Bad Request` – the value is not a valid IPv6 address, or the
///   body has fields other than `value`.
/// - `409 Conflict` – the address is already registered; the body carries its
///   `id` and `stix_id`.
/// - `500 Internal Server Error` – unexpected error.
#[post("/ipv6-addresses")]
pub async fn handler(
    state: web::Data<AppState>,
    body: web::Json<CreateIpv6AddressRequest>,
) -> impl Responder {
    debug!(value = %body.value, "POST /ipv6-addresses");

    let value = match Ipv6AddressValue::new(&body.value) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };
    let reference = Ipv6AddressReferenceResponse::from_id(&Ipv6AddressId::from_value(&value));

    let command = CreateIpv6AddressCommand { value };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<CreateIpv6AddressResponse>()
                .expect("Unexpected response type from CreateIpv6AddressCommandHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "AlreadyExists" => {
                        warn!(id = %reference.id, "IPv6 address already exists");
                        HttpResponse::Conflict().json(reference)
                    }
                    _ => {
                        warn!(id = %reference.id, error = %error.message, "Failed to create IPv6 address");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else {
                info!(id = %reference.id, "IPv6 address created");
                HttpResponse::Created()
                    .insert_header(("Location", format!("/ipv6-addresses/{}", reference.id)))
                    .json(reference)
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
