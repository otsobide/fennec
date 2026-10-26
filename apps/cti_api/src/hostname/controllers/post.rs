//! POST /hostnames handler for the CTI API.

use actix_web::{post, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::hostname::application::create_hostname::create_hostname_command::CreateHostnameCommand;
use kernel::hostname::application::create_hostname::create_hostname_response::CreateHostnameResponse;
use kernel::hostname::domain::value_objects::hostname_id::HostnameId;
use kernel::hostname::domain::value_objects::hostname_value::HostnameValue;

use crate::hostname::request_dtos::create_hostname_request::CreateHostnameRequest;
use crate::hostname::response_dtos::hostname_reference_response::HostnameReferenceResponse;
use crate::AppState;

/// Handles `POST /hostnames`.
///
/// # Responses
///
/// - `201 Created` – the hostname was registered; `Location` points at it and
///   the body carries its `id` and `stix_id`.
/// - `400 Bad Request` – the value is not a valid hostname, or the
///   body has fields other than `value`.
/// - `409 Conflict` – the hostname is already registered; the body carries its
///   `id` and `stix_id`.
/// - `500 Internal Server Error` – unexpected error.
#[post("/hostnames")]
pub async fn handler(
    state: web::Data<AppState>,
    body: web::Json<CreateHostnameRequest>,
) -> impl Responder {
    debug!(value = %body.value, "POST /hostnames");

    let value = match HostnameValue::new(&body.value) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };
    let reference = HostnameReferenceResponse::from_id(&HostnameId::from_value(&value));

    let command = CreateHostnameCommand { value };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<CreateHostnameResponse>()
                .expect("Unexpected response type from CreateHostnameCommandHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "AlreadyExists" => {
                        warn!(id = %reference.id, "Hostname already exists");
                        HttpResponse::Conflict().json(reference)
                    }
                    _ => {
                        warn!(id = %reference.id, error = %error.message, "Failed to create hostname");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else {
                info!(id = %reference.id, "Hostname created");
                HttpResponse::Created()
                    .insert_header(("Location", format!("/hostnames/{}", reference.id)))
                    .json(reference)
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
