//! GET /email-addresses/{id} handler for the CTI API.

use actix_web::{get, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::email_address::application::find_email_address::find_email_address_query::FindEmailAddressQuery;
use kernel::email_address::application::find_email_address::find_email_address_response::FindEmailAddressResponse;
use kernel::email_address::domain::value_objects::email_address_id::EmailAddressId;

use crate::email_address::response_dtos::email_address_response::EmailAddressResponse;
use crate::AppState;

/// Handles `GET /email-addresses/{id}`.
///
/// # Responses
///
/// - `200 OK` – the address, as an [`EmailAddressResponse`].
/// - `400 Bad Request` – the path id is not a UUID v5.
/// - `404 Not Found` – no address is registered under the id.
/// - `500 Internal Server Error` – unexpected error.
#[get("/email-addresses/{id}")]
pub async fn handler(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    debug!(id = %id_str, "GET /email-addresses/{{id}}");

    let id = match EmailAddressId::new(&id_str) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    find_and_respond(&state, id).await
}

/// Dispatches a [`FindEmailAddressQuery`] and maps its response to HTTP.
///
/// Shared with the lookup by value, which derives the id first.
pub(crate) async fn find_and_respond(state: &AppState, id: EmailAddressId) -> HttpResponse {
    let query = FindEmailAddressQuery { id };

    match state.query_bus.ask(Box::new(query)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<FindEmailAddressResponse>()
                .expect("Unexpected response type from FindEmailAddressQueryHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "NotFound" => {
                        info!(id = %id, "Email address not found");
                        HttpResponse::NotFound().body(error.message.clone())
                    }
                    _ => {
                        warn!(id = %id, error = %error.message, "Failed to find email address");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else if let Some(ref entry) = response.email_address {
                info!(id = %id, "Email address found");
                HttpResponse::Ok().json(EmailAddressResponse::from_entry(entry))
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(e) => {
            warn!(id = %id, error = %e, "Failed to find email address");
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}
