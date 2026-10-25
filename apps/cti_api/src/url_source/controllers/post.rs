//! POST /url-sources handler for the CTI API.

use actix_web::{post, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::url_source::application::create_url_source::create_url_source_command::CreateUrlSourceCommand;
use kernel::url_source::application::create_url_source::create_url_source_response::CreateUrlSourceResponse;
use kernel::url_source::domain::value_objects::url_source_format::UrlSourceFormat;
use kernel::url_source::domain::value_objects::url_source_id::UrlSourceId;
use kernel::url_source::domain::value_objects::url_source_polling_interval::UrlSourcePollingInterval;
use kernel::url_source::domain::value_objects::url_source_url::UrlSourceUrl;

use crate::url_source::request_dtos::create_url_source_request::CreateUrlSourceRequest;
use crate::AppState;

#[post("/url-sources")]
pub async fn handler(
    state: web::Data<AppState>,
    body: web::Json<CreateUrlSourceRequest>,
) -> impl Responder {
    debug!(id = %body.id, "POST /url-sources");

    let id = match UrlSourceId::new(&body.id) {
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

    let command = CreateUrlSourceCommand {
        id,
        url,
        format,
        polling_interval,
    };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<CreateUrlSourceResponse>()
                .expect("Unexpected response type from CreateUrlSourceCommandHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "AlreadyExists" => {
                        warn!(id = %body.id, "Url source already exists");
                        HttpResponse::Conflict().body(error.message.clone())
                    }
                    "NotFound" => HttpResponse::NotFound().body(error.message.clone()),
                    _ => {
                        warn!(id = %body.id, error = %error.message, "Failed to create url source");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else {
                info!(id = %body.id, "Url source created");
                HttpResponse::Created().finish()
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
