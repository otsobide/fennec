use actix_web::{dev::ServiceResponse, test, App};
use serde_json::json;

use cti_api::{build_state, configure_routes};

#[tokio::test]
async fn it_returns_201_when_url_source_is_created() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/url-sources")
        .set_json(json!({
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "url": "https://feeds.example.com/iocs.txt",
            "format": "plain",
            "polling_interval_seconds": 3600
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
        "url": "https://feeds.example.com/dup.txt",
        "format": "plain",
        "polling_interval_seconds": 600
    });

    let first = test::TestRequest::post()
        .uri("/url-sources")
        .set_json(&body)
        .to_request();
    let _: ServiceResponse = test::call_service(&app, first).await;

    let second = test::TestRequest::post()
        .uri("/url-sources")
        .set_json(&body)
        .to_request();
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
        .uri("/url-sources")
        .set_json(json!({
            "id": "not-a-uuid",
            "url": "https://feeds.example.com/iocs.txt",
            "format": "plain",
            "polling_interval_seconds": 600
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn it_returns_400_for_invalid_url_scheme() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/url-sources")
        .set_json(json!({
            "id": "550e8400-e29b-41d4-a716-446655440001",
            "url": "ftp://feeds.example.com/iocs.txt",
            "format": "plain",
            "polling_interval_seconds": 600
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn it_returns_400_for_unknown_format() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/url-sources")
        .set_json(json!({
            "id": "550e8400-e29b-41d4-a716-446655440002",
            "url": "https://feeds.example.com/iocs.txt",
            "format": "yaml",
            "polling_interval_seconds": 600
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn it_returns_400_for_zero_polling_interval() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/url-sources")
        .set_json(json!({
            "id": "550e8400-e29b-41d4-a716-446655440003",
            "url": "https://feeds.example.com/iocs.txt",
            "format": "plain",
            "polling_interval_seconds": 0
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}
