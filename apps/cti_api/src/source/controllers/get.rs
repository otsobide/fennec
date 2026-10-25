//! GET /sources/{id} handler for the CTI API.

use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;
use tracing::{debug, info, warn};

use kernel::source::application::find_source::find_source_query::FindSourceQuery;
use kernel::source::application::find_source::find_source_response::FindSourceResponse;
use kernel::source::domain::value_objects::source_id::SourceId;

use crate::AppState;

/// JSON response body for `GET /sources/{id}`.
#[derive(Serialize)]
pub struct GetSourceResponse {
    pub id: String,
    pub source_type: String,
    pub status: String,
    pub description: String,
    /// Seconds since the Unix epoch.
    pub created_at: u64,
    /// Seconds since the Unix epoch.
    pub updated_at: u64,
}

/// Handles `GET /sources/{id}`.
///
/// # Responses
///
/// - `200 OK` – source found.
/// - `400 Bad Request` – the path id is not a valid UUID.
/// - `404 Not Found` – no source exists for the given id.
/// - `500 Internal Server Error` – unexpected error.
#[get("/sources/{id}")]
pub async fn handler(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    debug!(id = %id_str, "GET /sources/{{id}}");

    let id = match SourceId::new(&id_str) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    let query = FindSourceQuery { id };

    match state.query_bus.ask(Box::new(query)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<FindSourceResponse>()
                .expect("Unexpected response type from FindSourceQueryHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "NotFound" => {
                        info!(id = %id_str, "Source not found");
                        HttpResponse::NotFound().body(error.message.clone())
                    }
                    _ => {
                        warn!(id = %id_str, error = %error.message, "Failed to find source");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else if let Some(ref entry) = response.source {
                info!(id = %id_str, "Source found");
                HttpResponse::Ok().json(GetSourceResponse {
                    id: entry.id.clone(),
                    source_type: entry.source_type.clone(),
                    status: entry.status.clone(),
                    description: entry.description.clone(),
                    created_at: to_unix_secs(entry.created_at),
                    updated_at: to_unix_secs(entry.updated_at),
                })
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(e) => {
            warn!(id = %id_str, error = %e, "Failed to find source");
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

fn to_unix_secs(time: SystemTime) -> u64 {
    time.duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
