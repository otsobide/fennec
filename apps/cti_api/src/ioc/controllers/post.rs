//! POST /iocs handler for the CTI API.

use actix_web::{post, web, HttpResponse, Responder};
use tracing::{debug, info, warn};

use ::ioc::ioc::application::create_ioc::create_ioc_command::CreateIocCommand;
use ::ioc::ioc::application::create_ioc::create_ioc_response::CreateIocResponse;
use ::ioc::ioc::domain::value_objects::ioc_id::IocId;
use ::ioc::ioc::domain::value_objects::ioc_type::IocType;
use ::ioc::ioc::domain::value_objects::ioc_value::IocValue;

use crate::ioc::request_dtos::create_ioc_request::CreateIocRequest;
use crate::AppState;

#[post("/iocs")]
pub async fn handler(
    state: web::Data<AppState>,
    body: web::Json<CreateIocRequest>,
) -> impl Responder {
    debug!(id = %body.id, "POST /iocs");

    let id = match IocId::new(&body.id) {
        Ok(value) => value,
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    };

    let ioc_type = match IocType::from_str(&body.ioc_type) {
        Ok(t) => t,
        Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
    };

    let value = match IocValue::new(body.value.clone()) {
        Ok(v) => v,
        Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
    };

    let command = CreateIocCommand {
        id,
        ioc_type,
        value,
    };

    match state.command_bus.dispatch(Box::new(command)).await {
        Ok(boxed) => {
            let response = boxed
                .downcast::<CreateIocResponse>()
                .expect("Unexpected response type from CreateIocCommandHandler");

            if let Some(ref error) = response.error {
                match error.concept.as_str() {
                    "AlreadyExists" => {
                        warn!(id = %body.id, "Ioc already exists");
                        HttpResponse::Conflict().body(error.message.clone())
                    }
                    "NotFound" => HttpResponse::NotFound().body(error.message.clone()),
                    _ => {
                        warn!(id = %body.id, error = %error.message, "Failed to create ioc");
                        HttpResponse::InternalServerError().body(error.message.clone())
                    }
                }
            } else {
                info!(id = %body.id, "Ioc created");
                HttpResponse::Created().finish()
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
