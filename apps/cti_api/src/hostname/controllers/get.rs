//! GET /hostnames/{id} handler for the CTI API.

use actix_web::{get, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::hostname::application::find_hostname::find_hostname_query::FindHostnameQuery;
use kernel::hostname::application::find_hostname::find_hostname_response::FindHostnameResponse;
use kernel::hostname::domain::value_objects::hostname_id::HostnameId;

use crate::hostname::response_dtos::hostname_response::HostnameResponse;
use crate::AppState;

/// Handles `GET /hostnames/{id}`.
///
/// # Responses
///
/// - `200 OK` – the hostname, as a [`HostnameResponse`].
/// - `400 Bad Request` – the path id is not a UUID v5.
/// - `404 Not Found` – no hostname is registered under the id.
/// - `500 Internal Server Error` – unexpected error.
#[get("/hostnames/{id}")]
pub async fn handler(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    debug!(id = %id_str, "GET /hostnames/{{id}}");

    let id = match HostnameId::new(&id_str) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    find_and_respond(&state, id).await
}

/// Dispatches a [`FindHostnameQuery`] and maps its response to HTTP.
///
/// Shared with the lookup by value, which derives the id first.
pub(crate) async fn find_and_respond(state: &AppState, id: HostnameId) -> HttpResponse {
    let query = FindHostnameQuery { id };

    match state.query_bus.ask(Box::new(query)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<FindHostnameResponse>()
                .expect("Unexpected response type from FindHostnameQueryHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "NotFound" => {
                        info!(id = %id, "Hostname not found");
                        HttpResponse::NotFound().body(error.message.clone())
                    }
                    _ => {
                        warn!(id = %id, error = %error.message, "Failed to find hostname");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else if let Some(ref entry) = response.hostname {
                info!(id = %id, "Hostname found");
                HttpResponse::Ok().json(HostnameResponse::from_entry(entry))
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(e) => {
            warn!(id = %id, error = %e, "Failed to find hostname");
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}
