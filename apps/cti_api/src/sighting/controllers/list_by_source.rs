//! GET /sources/{source_id}/sightings handler for the CTI API.

use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;
use tracing::{debug, info, warn};

use kernel::sighting::application::list_sightings_by_source::list_sightings_by_source_query::ListSightingsBySourceQuery;
use kernel::sighting::application::list_sightings_by_source::list_sightings_by_source_response::ListSightingsBySourceResponse;
use kernel::sighting::domain::value_objects::sighting_source_id::SightingSourceId;

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

#[get("/sources/{source_id}/sightings")]
pub async fn handler(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let source_id_str = path.into_inner();
    debug!(source_id = %source_id_str, "GET /sources/{{source_id}}/sightings");

    let source_id = match SightingSourceId::new(&source_id_str) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    let query = ListSightingsBySourceQuery { source_id };

    match state.query_bus.ask(Box::new(query)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<ListSightingsBySourceResponse>()
                .expect("Unexpected response type from ListSightingsBySourceQueryHandler");

            if let Some(ref error) = response.error {
                warn!(source_id = %source_id_str, error = %error.message, "Failed to list sightings by source");
                return HttpResponse::InternalServerError().body(error.message.clone());
            }

            info!(source_id = %source_id_str, count = response.sightings.len(), "Sightings listed by source");
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
            warn!(source_id = %source_id_str, error = %e, "Failed to list sightings by source");
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

fn to_unix_secs(time: SystemTime) -> u64 {
    time.duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
