//! DELETE /domain-names/{id} handler for the CTI API.

use actix_web::{delete, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::domain_name::application::delete_domain_name::delete_domain_name_command::DeleteDomainNameCommand;
use kernel::domain_name::application::delete_domain_name::delete_domain_name_response::DeleteDomainNameResponse;
use kernel::domain_name::domain::value_objects::domain_name_id::DomainNameId;

use crate::AppState;

/// Handles `DELETE /domain-names/{id}`.
///
/// # Responses
///
/// - `204 No Content` – the name was deleted.
/// - `400 Bad Request` – the path id is not a UUID v5.
/// - `404 Not Found` – no name is registered under the id.
/// - `500 Internal Server Error` – unexpected error.
#[delete("/domain-names/{id}")]
pub async fn handler(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    debug!(id = %id_str, "DELETE /domain-names/{{id}}");

    let id = match DomainNameId::new(&id_str) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    let command = DeleteDomainNameCommand { id };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<DeleteDomainNameResponse>()
                .expect("Unexpected response type from DeleteDomainNameCommandHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "NotFound" => {
                        warn!(id = %id_str, "Domain name not found for deletion");
                        HttpResponse::NotFound().body(error.message.clone())
                    }
                    _ => {
                        warn!(id = %id_str, error = %error.message, "Failed to delete domain name");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else {
                info!(id = %id_str, "Domain name deleted");
                HttpResponse::NoContent().finish()
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
