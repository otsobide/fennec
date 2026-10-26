//! POST /email-addresses handler for the CTI API.

use actix_web::{post, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::email_address::application::create_email_address::create_email_address_command::CreateEmailAddressCommand;
use kernel::email_address::application::create_email_address::create_email_address_response::CreateEmailAddressResponse;
use kernel::email_address::domain::value_objects::email_address_id::EmailAddressId;
use kernel::email_address::domain::value_objects::email_address_value::EmailAddressValue;

use crate::email_address::request_dtos::create_email_address_request::CreateEmailAddressRequest;
use crate::email_address::response_dtos::email_address_reference_response::EmailAddressReferenceResponse;
use crate::AppState;

/// Handles `POST /email-addresses`.
///
/// # Responses
///
/// - `201 Created` – the address was registered; `Location` points at it and
///   the body carries its `id` and `stix_id`.
/// - `400 Bad Request` – the value is not a valid email address, or the
///   body has fields other than `value`.
/// - `409 Conflict` – the address is already registered; the body carries its
///   `id` and `stix_id`.
/// - `500 Internal Server Error` – unexpected error.
#[post("/email-addresses")]
pub async fn handler(
    state: web::Data<AppState>,
    body: web::Json<CreateEmailAddressRequest>,
) -> impl Responder {
    debug!(value = %body.value, "POST /email-addresses");

    let value = match EmailAddressValue::new(&body.value) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };
    let reference = EmailAddressReferenceResponse::from_id(&EmailAddressId::from_value(&value));

    let command = CreateEmailAddressCommand { value };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<CreateEmailAddressResponse>()
                .expect("Unexpected response type from CreateEmailAddressCommandHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "AlreadyExists" => {
                        warn!(id = %reference.id, "Email address already exists");
                        HttpResponse::Conflict().json(reference)
                    }
                    _ => {
                        warn!(id = %reference.id, error = %error.message, "Failed to create email address");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else {
                info!(id = %reference.id, "Email address created");
                HttpResponse::Created()
                    .insert_header(("Location", format!("/email-addresses/{}", reference.id)))
                    .json(reference)
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
