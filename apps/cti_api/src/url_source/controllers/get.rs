//! GET /url-sources/{id} handler for the CTI API.

use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;
use tracing::{debug, info, warn};
use uuid::Uuid;

use kernel::url_source::application::find_url_source::find_url_source_query::FindUrlSourceQuery;
use kernel::url_source::application::find_url_source::find_url_source_response::FindUrlSourceResponse;
use kernel::url_source::domain::value_objects::url_source_id::UrlSourceId;

use crate::AppState;

#[derive(Serialize)]
pub struct GetUrlSourceResponse {
    pub id: String,
    pub url: String,
    pub format: String,
    pub polling_interval_seconds: u32,
    pub created_at: u64,
    pub updated_at: u64,
}

#[get("/url-sources/{id}")]
pub async fn handler(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let id_str = path.into_inner();
    debug!(id = %id_str, "GET /url-sources/{{id}}");

    let id = match Uuid::parse_str(&id_str) {
        Ok(uuid) => UrlSourceId::from_uuid(uuid),
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format"),
    };

    let query = FindUrlSourceQuery { id };

    match state.query_bus.ask(Box::new(query)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<FindUrlSourceResponse>()
                .expect("Unexpected response type from FindUrlSourceQueryHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "NotFound" => {
                        info!(id = %id_str, "Url source not found");
                        HttpResponse::NotFound().body(error.message.clone())
                    }
                    _ => {
                        warn!(id = %id_str, error = %error.message, "Failed to find url source");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else if let Some(ref entry) = response.url_source {
                info!(id = %id_str, "Url source found");
                HttpResponse::Ok().json(GetUrlSourceResponse {
                    id: entry.id.clone(),
                    url: entry.url.clone(),
                    format: entry.format.clone(),
                    polling_interval_seconds: entry.polling_interval_seconds,
                    created_at: to_unix_secs(entry.created_at),
                    updated_at: to_unix_secs(entry.updated_at),
                })
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(e) => {
            warn!(id = %id_str, error = %e, "Failed to find url source");
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

fn to_unix_secs(time: SystemTime) -> u64 {
    time.duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}
