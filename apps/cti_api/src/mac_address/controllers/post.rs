//! POST /mac-addresses handler for the CTI API.

use actix_web::{post, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::mac_address::application::create_mac_address::create_mac_address_command::CreateMacAddressCommand;
use kernel::mac_address::application::create_mac_address::create_mac_address_response::CreateMacAddressResponse;
use kernel::mac_address::domain::value_objects::mac_address_id::MacAddressId;
use kernel::mac_address::domain::value_objects::mac_address_value::MacAddressValue;

use crate::mac_address::request_dtos::create_mac_address_request::CreateMacAddressRequest;
use crate::mac_address::response_dtos::mac_address_reference_response::MacAddressReferenceResponse;
use crate::AppState;

/// Handles `POST /mac-addresses`.
///
/// # Responses
///
/// - `201 Created` – the address was registered; `Location` points at it and
///   the body carries its `id` and `stix_id`.
/// - `400 Bad Request` – the value is not a valid MAC address, or the
///   body has fields other than `value`.
/// - `409 Conflict` – the address is already registered; the body carries its
///   `id` and `stix_id`.
/// - `500 Internal Server Error` – unexpected error.
#[post("/mac-addresses")]
pub async fn handler(
    state: web::Data<AppState>,
    body: web::Json<CreateMacAddressRequest>,
) -> impl Responder {
    debug!(value = %body.value, "POST /mac-addresses");

    let value = match MacAddressValue::new(&body.value) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };
    let reference = MacAddressReferenceResponse::from_id(&MacAddressId::from_value(&value));

    let command = CreateMacAddressCommand { value };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<CreateMacAddressResponse>()
                .expect("Unexpected response type from CreateMacAddressCommandHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "AlreadyExists" => {
                        warn!(id = %reference.id, "MAC address already exists");
                        HttpResponse::Conflict().json(reference)
                    }
                    _ => {
                        warn!(id = %reference.id, error = %error.message, "Failed to create MAC address");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else {
                info!(id = %reference.id, "MAC address created");
                HttpResponse::Created()
                    .insert_header(("Location", format!("/mac-addresses/{}", reference.id)))
                    .json(reference)
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
