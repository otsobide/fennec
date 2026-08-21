//! PUT /url-sources/{id} handler for the CTI API.

use actix_web::{put, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::url_source::application::update_url_source::update_url_source_command::UpdateUrlSourceCommand;
use kernel::url_source::application::update_url_source::update_url_source_response::UpdateUrlSourceResponse;
use kernel::url_source::domain::value_objects::url_source_format::UrlSourceFormat;
use kernel::url_source::domain::value_objects::url_source_id::UrlSourceId;
use kernel::url_source::domain::value_objects::url_source_polling_interval::UrlSourcePollingInterval;
use kernel::url_source::domain::value_objects::url_source_url::UrlSourceUrl;

use crate::url_source::request_dtos::update_url_source_request::UpdateUrlSourceRequest;
use crate::AppState;

#[put("/url-sources/{id}")]
pub async fn handler(
    state: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<UpdateUrlSourceRequest>,
) -> impl Responder {
    let id_str = path.into_inner();
    debug!(id = %id_str, "PUT /url-sources/{{id}}");

    let id = match UrlSourceId::new(&id_str) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    let url = match UrlSourceUrl::new(body.url.clone()) {
        Ok(u) => u,
        Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
    };

    let format = match UrlSourceFormat::from_str(&body.format) {
        Ok(f) => f,
        Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
    };

    let polling_interval =
        match UrlSourcePollingInterval::from_seconds(body.polling_interval_seconds) {
            Ok(p) => p,
            Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
        };

    let command = UpdateUrlSourceCommand {
        id,
        url,
        format,
        polling_interval,
    };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<UpdateUrlSourceResponse>()
                .expect("Unexpected response type from UpdateUrlSourceCommandHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "NotFound" => {
                        warn!(id = %id_str, "Url source not found for update");
                        HttpResponse::NotFound().body(error.message.clone())
                    }
                    "AlreadyExists" => HttpResponse::Conflict().body(error.message.clone()),
                    _ => {
                        warn!(id = %id_str, error = %error.message, "Failed to update url source");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else {
                info!(id = %id_str, "Url source updated");
                HttpResponse::Ok().finish()
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
