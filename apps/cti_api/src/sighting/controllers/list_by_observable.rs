//! GET /observables/{kind}/{id}/sightings handler for the CTI API.

use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;
use tracing::{debug, info, warn};

use kernel::sighting::application::list_sightings_by_observable::list_sightings_by_observable_query::ListSightingsByObservableQuery;
use kernel::sighting::application::list_sightings_by_observable::list_sightings_by_observable_response::ListSightingsByObservableResponse;
use kernel::sighting::domain::value_objects::sighting_observable_id::SightingObservableId;
use kernel::sighting::domain::value_objects::sighting_observable_kind::SightingObservableKind;

use crate::AppState;

#[derive(Serialize)]
pub struct SightingListItem {
    pub id: String,
    pub observable_kind: String,
    pub observable_id: String,
    pub source_id: String,
    pub first_seen: u64,
    pub last_seen: u64,
    pub count: u64,
    pub created_at: u64,
    pub updated_at: u64,
}

/// Handles `GET /observables/{kind}/{id}/sightings`: every sighting of one
/// observable, identified by its module kind and id.
///
/// # Responses
///
/// - `200 OK` – the sightings, possibly none.
/// - `400 Bad Request` – the kind is not a module name or the id is not a
///   UUID v4 or v5.
/// - `500 Internal Server Error` – unexpected error.
#[get("/observables/{kind}/{id}/sightings")]
pub async fn handler(
    state: web::Data<AppState>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let (kind_str, id_str) = path.into_inner();
    debug!(kind = %kind_str, id = %id_str, "GET /observables/{{kind}}/{{id}}/sightings");

    let observable_kind = match SightingObservableKind::new(&kind_str) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    let observable_id = match SightingObservableId::new(&id_str) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    let query = ListSightingsByObservableQuery {
        observable_kind,
        observable_id,
    };

    match state.query_bus.ask(Box::new(query)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<ListSightingsByObservableResponse>()
                .expect("Unexpected response type from ListSightingsByObservableQueryHandler");

            if let Some(ref error) = response.error {
                warn!(kind = %kind_str, id = %id_str, error = %error.message, "Failed to list sightings by observable");
                return HttpResponse::InternalServerError().body(error.message.clone());
            }

            info!(kind = %kind_str, id = %id_str, count = response.sightings.len(), "Sightings listed by observable");
            let items: Vec<SightingListItem> = response
                .sightings
                .iter()
                .map(|entry| SightingListItem {
                    id: entry.id.clone(),
                    observable_kind: entry.observable_kind.clone(),
                    observable_id: entry.observable_id.clone(),
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
            warn!(kind = %kind_str, id = %id_str, error = %e, "Failed to list sightings by observable");
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

fn to_unix_secs(time: SystemTime) -> u64 {
    time.duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
