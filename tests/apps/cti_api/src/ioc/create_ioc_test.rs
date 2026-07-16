use actix_web::{dev::ServiceResponse, test, App};
use serde_json::json;

use cti_api::{build_state, configure_routes};

#[tokio::test]
async fn it_returns_201_when_ioc_is_created() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/iocs")
        .set_json(json!({
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "ioc_type": "ipv4",
            "value": "203.0.113.10"
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);
}

#[tokio::test]
async fn it_returns_409_when_creating_a_duplicate_id() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let body = json!({
        "id": "deadbeef-dead-beef-dead-beefdeadbeef",
        "ioc_type": "domain",
        "value": "malicious.example.com"
    });

    let first = test::TestRequest::post().uri("/iocs").set_json(&body).to_request();
    let _: ServiceResponse = test::call_service(&app, first).await;

    let second = test::TestRequest::post().uri("/iocs").set_json(&body).to_request();
    let resp: ServiceResponse = test::call_service(&app, second).await;

    assert_eq!(resp.status(), 409);
}

#[tokio::test]
async fn it_returns_400_for_invalid_uuid() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/iocs")
        .set_json(json!({
            "id": "not-a-uuid",
            "ioc_type": "ipv4",
            "value": "203.0.113.10"
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn it_returns_400_for_unknown_ioc_type() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/iocs")
        .set_json(json!({
            "id": "550e8400-e29b-41d4-a716-446655440001",
            "ioc_type": "cidr",
            "value": "203.0.113.0/24"
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn it_returns_400_for_empty_value() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/iocs")
        .set_json(json!({
            "id": "550e8400-e29b-41d4-a716-446655440002",
            "ioc_type": "ipv4",
            "value": ""
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}
