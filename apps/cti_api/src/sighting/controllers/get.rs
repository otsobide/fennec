//! GET /sightings/{id} handler for the CTI API.

use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;
use tracing::{debug, info, warn};
use uuid::Uuid;

use ::sighting::sighting::application::find_sighting::find_sighting_query::FindSightingQuery;
use ::sighting::sighting::application::find_sighting::find_sighting_response::FindSightingResponse;
use ::sighting::sighting::domain::value_objects::sighting_id::SightingId;

use crate::AppState;

#[derive(Serialize)]
pub struct GetSightingResponse {
    pub id: String,
    pub ioc_id: String,
    pub source_id: String,
    pub first_seen: u64,
    pub last_seen: u64,
    pub count: u64,
    pub created_at: u64,
    pub updated_at: u64,
}

#[get("/sightings/{id}")]
pub async fn handler(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let id_str = path.into_inner();
    debug!(id = %id_str, "GET /sightings/{{id}}");

    let id = match Uuid::parse_str(&id_str) {
        Ok(uuid) => SightingId::from_uuid(uuid),
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format"),
    };

    let query = FindSightingQuery { id };

    match state.query_bus.ask(Box::new(query)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<FindSightingResponse>()
                .expect("Unexpected response type from FindSightingQueryHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "NotFound" => {
                        info!(id = %id_str, "Sighting not found");
                        HttpResponse::NotFound().body(error.message.clone())
                    }
                    _ => {
                        warn!(id = %id_str, error = %error.message, "Failed to find sighting");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else if let Some(ref entry) = response.sighting {
                info!(id = %id_str, "Sighting found");
                HttpResponse::Ok().json(GetSightingResponse {
                    id: entry.id.clone(),
                    ioc_id: entry.ioc_id.clone(),
                    source_id: entry.source_id.clone(),
                    first_seen: to_unix_secs(entry.first_seen),
                    last_seen: to_unix_secs(entry.last_seen),
                    count: entry.count,
                    created_at: to_unix_secs(entry.created_at),
                    updated_at: to_unix_secs(entry.updated_at),
                })
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(e) => {
            warn!(id = %id_str, error = %e, "Failed to find sighting");
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

fn to_unix_secs(time: SystemTime) -> u64 {
    time.duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}
