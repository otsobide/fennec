//! POST /dns-resolutions handler for the CTI API.

use actix_web::{post, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use kernel::dns_resolution::application::record_dns_resolution::record_dns_resolution_command::RecordDnsResolutionCommand;
use kernel::dns_resolution::application::record_dns_resolution::record_dns_resolution_response::RecordDnsResolutionResponse;
use kernel::dns_resolution::domain::value_objects::dns_resolution_answer::DnsResolutionAnswer;
use kernel::dns_resolution::domain::value_objects::dns_resolution_count::DnsResolutionCount;
use kernel::dns_resolution::domain::value_objects::dns_resolution_source_id::DnsResolutionSourceId;
use kernel::dns_resolution::domain::value_objects::dns_resolution_subject::DnsResolutionSubject;
use kernel::dns_resolution::domain::value_objects::dns_resolution_window::DnsResolutionWindow;

use crate::dns_resolution::request_dtos::record_dns_resolution_request::RecordDnsResolutionRequest;
use crate::dns_resolution::response_dtos::dns_resolution_recording_response::DnsResolutionRecordingResponse;
use crate::AppState;

/// Handles `POST /dns-resolutions`: records a source's report that a domain
/// name resolved to an address.
///
/// The id is derived from the endpoints, so every source and every retry of
/// the same resolution converges on one aggregate; the report is merged
/// into the source's observation.
///
/// # Responses
///
/// - `201 Created` – the resolution was not recorded yet; `Location` points
///   at it and the body is
///   `{ "id", "stix_id", "relationship_type", "outcome": "created" }`.
/// - `200 OK` – the resolution was recorded already; the body is
///   `{ "id", "stix_id", "relationship_type", "outcome" }` with `observed`
///   when the report added or widened the source's observation and
///   `unchanged` when it added nothing.
/// - `400 Bad Request` – a value is invalid, neither or both of
///   `ipv4_address` and `ipv6_address` are given, or the body has an
///   unknown field.
/// - `409 Conflict` – a concurrent request recorded the same resolution
///   first; retrying records the report.
/// - `500 Internal Server Error` – unexpected error.
#[post("/dns-resolutions")]
pub async fn handler(
    state: web::Data<AppState>,
    body: web::Json<RecordDnsResolutionRequest>,
) -> impl Responder {
    debug!(
        domain_name = %body.domain_name,
        source_id = %body.source_id,
        "POST /dns-resolutions"
    );

    let subject = match DnsResolutionSubject::domain_name(&body.domain_name) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };
    let answer = match (&body.ipv4_address, &body.ipv6_address) {
        (Some(address), None) => DnsResolutionAnswer::ipv4_address(address),
        (None, Some(address)) => DnsResolutionAnswer::ipv6_address(address),
        _ => {
            return HttpResponse::BadRequest()
                .body("exactly one of ipv4_address or ipv6_address is required")
        }
    };
    let answer = match answer {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };
    let source_id = match DnsResolutionSourceId::new(&body.source_id) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };
    let window = match DnsResolutionWindow::from_unix_seconds(body.first_seen, body.last_seen) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };
    let count = match body.count.map(DnsResolutionCount::from_u64).transpose() {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    let command = RecordDnsResolutionCommand {
        subject,
        answer,
        source_id,
        window,
        count,
    };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<RecordDnsResolutionResponse>()
                .expect("Unexpected response type from RecordDnsResolutionCommandHandler");

            if let Some(ref error) = response.error {
                return match error.concept.as_str() {
                    "AlreadyExists" => {
                        warn!(domain_name = %body.domain_name, "DNS resolution recorded concurrently");
                        HttpResponse::Conflict().body(error.message.clone())
                    }
                    _ => {
                        warn!(domain_name = %body.domain_name, error = %error.message, "Failed to record DNS resolution");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                };
            }

            let (Some(id), Some(stix_id), Some(relationship_type), Some(outcome)) = (
                response.id.clone(),
                response.stix_id.clone(),
                response.relationship_type.clone(),
                response.outcome.clone(),
            ) else {
                warn!(domain_name = %body.domain_name, "DNS resolution recorded without its id, STIX id, relationship type or outcome");
                return HttpResponse::InternalServerError().finish();
            };
            info!(id = %id, outcome = %outcome, "DNS resolution recorded");

            let created = outcome == "created";
            let recording = DnsResolutionRecordingResponse {
                id: id.clone(),
                stix_id,
                relationship_type,
                outcome,
            };
            if created {
                HttpResponse::Created()
                    .insert_header(("Location", format!("/dns-resolutions/{id}")))
                    .json(recording)
            } else {
                HttpResponse::Ok().json(recording)
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
