//! GET /iocs/{ioc_id}/sightings handler for the CTI API.

use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;
use tracing::{debug, info, warn};
use uuid::Uuid;

use ::sighting::sighting::application::list_sightings_by_ioc::list_sightings_by_ioc_query::ListSightingsByIocQuery;
use ::sighting::sighting::application::list_sightings_by_ioc::list_sightings_by_ioc_response::ListSightingsByIocResponse;
use ::sighting::sighting::domain::value_objects::sighting_ioc_id::SightingIocId;

use crate::AppState;

#[derive(Serialize)]
pub struct SightingListItem {
    pub id: String,
    pub ioc_id: String,
    pub source_id: String,
    pub first_seen: u64,
    pub last_seen: u64,
    pub count: u64,
    pub created_at: u64,
    pub updated_at: u64,
}

#[get("/iocs/{ioc_id}/sightings")]
pub async fn handler(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let ioc_id_str = path.into_inner();
    debug!(ioc_id = %ioc_id_str, "GET /iocs/{{ioc_id}}/sightings");

    let ioc_id = match Uuid::parse_str(&ioc_id_str) {
        Ok(uuid) => SightingIocId::from_uuid(uuid),
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format"),
    };

    let query = ListSightingsByIocQuery { ioc_id };

    match state.query_bus.ask(Box::new(query)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<ListSightingsByIocResponse>()
                .expect("Unexpected response type from ListSightingsByIocQueryHandler");

            if let Some(ref error) = response.error {
                warn!(ioc_id = %ioc_id_str, error = %error.message, "Failed to list sightings by ioc");
                return HttpResponse::InternalServerError().body(error.message.clone());
            }

            info!(ioc_id = %ioc_id_str, count = response.sightings.len(), "Sightings listed by ioc");
            let items: Vec<SightingListItem> = response
                .sightings
                .iter()
                .map(|entry| SightingListItem {
                    id: entry.id.clone(),
                    ioc_id: entry.ioc_id.clone(),
                    source_id: entry.source_id.clone(),
                    first_seen: to_unix_secs(entry.first_seen),
                    last_seen: to_unix_secs(entry.last_seen),
                    count: entry.count,
                    created_at: to_unix_secs(entry.created_at),
                    updated_at: to_unix_secs(entry.updated_at),
                })
                .collect();
            HttpResponse::Ok().json(items)
        }
        Err(e) => {
            warn!(ioc_id = %ioc_id_str, error = %e, "Failed to list sightings by ioc");
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

fn to_unix_secs(time: SystemTime) -> u64 {
    time.duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}
