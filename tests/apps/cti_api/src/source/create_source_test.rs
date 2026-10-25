use actix_web::{dev::ServiceResponse, test, App};
use serde_json::json;

use cti_api::{build_state, configure_routes};

#[tokio::test]
async fn it_returns_201_when_source_is_created() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/sources")
        .set_json(json!({
            "id": "550e8400-e29b-41d4-8716-446655440000",
            "source_type": "url",
            "status": "active",
            "description": "Primary feed"
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);
}

#[tokio::test]
async fn it_returns_409_when_creating_a_duplicate_id() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let body = json!({
        "id": "deadbeef-dead-4eef-8ead-beefdeadbeef",
        "source_type": "url",
        "status": "active",
        "description": "Dup test"
    });

    let first = test::TestRequest::post()
        .uri("/sources")
        .set_json(&body)
        .to_request();
    let _: ServiceResponse = test::call_service(&app, first).await;

    let second = test::TestRequest::post()
        .uri("/sources")
        .set_json(&body)
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, second).await;

    assert_eq!(resp.status(), 409);
}

#[tokio::test]
async fn it_returns_400_for_invalid_uuid() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/sources")
        .set_json(json!({
            "id": "not-a-uuid",
            "source_type": "url",
            "status": "active",
            "description": "bad id"
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn it_returns_400_for_unknown_source_type() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/sources")
        .set_json(json!({
            "id": "550e8400-e29b-41d4-8716-446655440001",
            "source_type": "rss",
            "status": "active",
            "description": "bad type"
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn it_returns_400_for_unknown_status() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/sources")
        .set_json(json!({
            "id": "550e8400-e29b-41d4-8716-446655440002",
            "source_type": "url",
            "status": "paused",
            "description": "bad status"
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}
