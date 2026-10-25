//! POST /ipv4-addresses handler for the CTI API.

use actix_web::{post, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::ipv4_address::application::create_ipv4_address::create_ipv4_address_command::CreateIpv4AddressCommand;
use kernel::ipv4_address::application::create_ipv4_address::create_ipv4_address_response::CreateIpv4AddressResponse;
use kernel::ipv4_address::domain::value_objects::ipv4_address_id::Ipv4AddressId;
use kernel::ipv4_address::domain::value_objects::ipv4_address_value::Ipv4AddressValue;

use crate::ipv4_address::request_dtos::create_ipv4_address_request::CreateIpv4AddressRequest;
use crate::ipv4_address::response_dtos::ipv4_address_reference_response::Ipv4AddressReferenceResponse;
use crate::AppState;

/// Handles `POST /ipv4-addresses`.
///
/// # Responses
///
/// - `201 Created` – the address was registered; `Location` points at it and
///   the body carries its `id` and `stix_id`.
/// - `400 Bad Request` – the value is not a single IPv4 host address, or the
///   body has fields other than `value`.
/// - `409 Conflict` – the address is already registered; the body carries its
///   `id` and `stix_id`.
/// - `500 Internal Server Error` – unexpected error.
#[post("/ipv4-addresses")]
pub async fn handler(
    state: web::Data<AppState>,
    body: web::Json<CreateIpv4AddressRequest>,
) -> impl Responder {
    debug!(value = %body.value, "POST /ipv4-addresses");

    let value = match Ipv4AddressValue::new(&body.value) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };
    let reference = Ipv4AddressReferenceResponse::from_id(&Ipv4AddressId::from_value(&value));

    let command = CreateIpv4AddressCommand { value };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<CreateIpv4AddressResponse>()
                .expect("Unexpected response type from CreateIpv4AddressCommandHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "AlreadyExists" => {
                        warn!(id = %reference.id, "Ipv4 address already exists");
                        HttpResponse::Conflict().json(reference)
                    }
                    _ => {
                        warn!(id = %reference.id, error = %error.message, "Failed to create ipv4 address");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else {
                info!(id = %reference.id, "Ipv4 address created");
                HttpResponse::Created()
                    .insert_header(("Location", format!("/ipv4-addresses/{}", reference.id)))
                    .json(reference)
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
