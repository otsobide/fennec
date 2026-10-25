//! GET /iocs/{id} handler for the CTI API.

use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;
use tracing::{debug, info, warn};

use kernel::ioc::application::find_ioc::find_ioc_query::FindIocQuery;
use kernel::ioc::application::find_ioc::find_ioc_response::FindIocResponse;
use kernel::ioc::domain::value_objects::ioc_id::IocId;

use crate::AppState;

#[derive(Serialize)]
pub struct GetIocResponse {
    pub id: String,
    pub ioc_type: String,
    pub value: String,
    pub created_at: u64,
    pub updated_at: u64,
}

#[get("/iocs/{id}")]
pub async fn handler(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    debug!(id = %id_str, "GET /iocs/{{id}}");

    let id = match IocId::new(&id_str) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    let query = FindIocQuery { id };

    match state.query_bus.ask(Box::new(query)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<FindIocResponse>()
                .expect("Unexpected response type from FindIocQueryHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "NotFound" => {
                        info!(id = %id_str, "Ioc not found");
                        HttpResponse::NotFound().body(error.message.clone())
                    }
                    _ => {
                        warn!(id = %id_str, error = %error.message, "Failed to find ioc");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else if let Some(ref entry) = response.ioc {
                info!(id = %id_str, "Ioc found");
                HttpResponse::Ok().json(GetIocResponse {
                    id: entry.id.clone(),
                    ioc_type: entry.ioc_type.clone(),
                    value: entry.value.clone(),
                    created_at: to_unix_secs(entry.created_at),
                    updated_at: to_unix_secs(entry.updated_at),
                })
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(e) => {
            warn!(id = %id_str, error = %e, "Failed to find ioc");
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

fn to_unix_secs(time: SystemTime) -> u64 {
    time.duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
