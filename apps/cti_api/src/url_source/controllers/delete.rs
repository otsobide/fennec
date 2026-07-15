//! DELETE /url-sources/{id} handler for the CTI API.

use actix_web::{delete, web, HttpResponse, Responder};
use tracing::{debug, info, warn};
use uuid::Uuid;

use kernel::url_source::application::delete_url_source::delete_url_source_command::DeleteUrlSourceCommand;
use kernel::url_source::application::delete_url_source::delete_url_source_response::DeleteUrlSourceResponse;
use kernel::url_source::domain::value_objects::url_source_id::UrlSourceId;

use crate::AppState;

#[delete("/url-sources/{id}")]
pub async fn handler(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let id_str = path.into_inner();
    debug!(id = %id_str, "DELETE /url-sources/{{id}}");

    let id = match Uuid::parse_str(&id_str) {
        Ok(uuid) => UrlSourceId::from_uuid(uuid),
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format"),
    };

    let command = DeleteUrlSourceCommand { id };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<DeleteUrlSourceResponse>()
                .expect("Unexpected response type from DeleteUrlSourceCommandHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "NotFound" => {
                        warn!(id = %id_str, "Url source not found for deletion");
                        HttpResponse::NotFound().body(error.message.clone())
                    }
                    _ => {
                        warn!(id = %id_str, error = %error.message, "Failed to delete url source");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else {
                info!(id = %id_str, "Url source deleted");
                HttpResponse::NoContent().finish()
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
