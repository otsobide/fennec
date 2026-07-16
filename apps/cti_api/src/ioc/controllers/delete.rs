//! DELETE /iocs/{id} handler for the CTI API.

use actix_web::{delete, web, HttpResponse, Responder};
use tracing::{debug, info, warn};
use uuid::Uuid;

use ::ioc::ioc::application::delete_ioc::delete_ioc_command::DeleteIocCommand;
use ::ioc::ioc::application::delete_ioc::delete_ioc_response::DeleteIocResponse;
use ::ioc::ioc::domain::value_objects::ioc_id::IocId;

use crate::AppState;

#[delete("/iocs/{id}")]
pub async fn handler(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let id_str = path.into_inner();
    debug!(id = %id_str, "DELETE /iocs/{{id}}");

    let id = match Uuid::parse_str(&id_str) {
        Ok(uuid) => IocId::from_uuid(uuid),
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format"),
    };

    let command = DeleteIocCommand { id };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<DeleteIocResponse>()
                .expect("Unexpected response type from DeleteIocCommandHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "NotFound" => {
                        warn!(id = %id_str, "Ioc not found for deletion");
                        HttpResponse::NotFound().body(error.message.clone())
                    }
                    _ => {
                        warn!(id = %id_str, error = %error.message, "Failed to delete ioc");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else {
                info!(id = %id_str, "Ioc deleted");
                HttpResponse::NoContent().finish()
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
