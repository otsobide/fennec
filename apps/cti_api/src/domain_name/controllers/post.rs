//! POST /domain-names handler for the CTI API.

use actix_web::{post, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::domain_name::application::create_domain_name::create_domain_name_command::CreateDomainNameCommand;
use kernel::domain_name::application::create_domain_name::create_domain_name_response::CreateDomainNameResponse;
use kernel::domain_name::domain::value_objects::domain_name_id::DomainNameId;
use kernel::domain_name::domain::value_objects::domain_name_value::DomainNameValue;

use crate::domain_name::request_dtos::create_domain_name_request::CreateDomainNameRequest;
use crate::domain_name::response_dtos::domain_name_reference_response::DomainNameReferenceResponse;
use crate::AppState;

/// Handles `POST /domain-names`.
///
/// # Responses
///
/// - `201 Created` – the name was registered; `Location` points at it and
///   the body carries its `id` and `stix_id`.
/// - `400 Bad Request` – the value is not a valid domain name, or the
///   body has fields other than `value`.
/// - `409 Conflict` – the name is already registered; the body carries its
///   `id` and `stix_id`.
/// - `500 Internal Server Error` – unexpected error.
#[post("/domain-names")]
pub async fn handler(
    state: web::Data<AppState>,
    body: web::Json<CreateDomainNameRequest>,
) -> impl Responder {
    debug!(value = %body.value, "POST /domain-names");

    let value = match DomainNameValue::new(&body.value) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };
    let reference = DomainNameReferenceResponse::from_id(&DomainNameId::from_value(&value));

    let command = CreateDomainNameCommand { value };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<CreateDomainNameResponse>()
                .expect("Unexpected response type from CreateDomainNameCommandHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "AlreadyExists" => {
                        warn!(id = %reference.id, "Domain name already exists");
                        HttpResponse::Conflict().json(reference)
                    }
                    _ => {
                        warn!(id = %reference.id, error = %error.message, "Failed to create domain name");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else {
                info!(id = %reference.id, "Domain name created");
                HttpResponse::Created()
                    .insert_header(("Location", format!("/domain-names/{}", reference.id)))
                    .json(reference)
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
