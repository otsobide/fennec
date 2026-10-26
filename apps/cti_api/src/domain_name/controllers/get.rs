//! GET /domain-names/{id} handler for the CTI API.

use actix_web::{get, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::domain_name::application::find_domain_name::find_domain_name_query::FindDomainNameQuery;
use kernel::domain_name::application::find_domain_name::find_domain_name_response::FindDomainNameResponse;
use kernel::domain_name::domain::value_objects::domain_name_id::DomainNameId;

use crate::domain_name::response_dtos::domain_name_response::DomainNameResponse;
use crate::AppState;

/// Handles `GET /domain-names/{id}`.
///
/// # Responses
///
/// - `200 OK` – the name, as an [`DomainNameResponse`].
/// - `400 Bad Request` – the path id is not a UUID v5.
/// - `404 Not Found` – no name is registered under the id.
/// - `500 Internal Server Error` – unexpected error.
#[get("/domain-names/{id}")]
pub async fn handler(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    debug!(id = %id_str, "GET /domain-names/{{id}}");

    let id = match DomainNameId::new(&id_str) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    find_and_respond(&state, id).await
}

/// Dispatches a [`FindDomainNameQuery`] and maps its response to HTTP.
///
/// Shared with the lookup by value, which derives the id first.
pub(crate) async fn find_and_respond(state: &AppState, id: DomainNameId) -> HttpResponse {
    let query = FindDomainNameQuery { id };

    match state.query_bus.ask(Box::new(query)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<FindDomainNameResponse>()
                .expect("Unexpected response type from FindDomainNameQueryHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "NotFound" => {
                        info!(id = %id, "Domain name not found");
                        HttpResponse::NotFound().body(error.message.clone())
                    }
                    _ => {
                        warn!(id = %id, error = %error.message, "Failed to find domain name");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else if let Some(ref entry) = response.domain_name {
                info!(id = %id, "Domain name found");
                HttpResponse::Ok().json(DomainNameResponse::from_entry(entry))
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(e) => {
            warn!(id = %id, error = %e, "Failed to find domain name");
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}
