use actix_web::{dev::ServiceResponse, test, App};
use serde_json::json;

use cti_api::{build_state, configure_routes};

#[tokio::test]
async fn it_returns_201_with_the_derived_id_and_stix_id() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/ipv4-networks")
        .set_json(json!({ "value": "198.51.100.0/24" }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);
    assert_eq!(
        resp.headers().get("Location").unwrap(),
        "/ipv4-networks/1ea47840-aa39-5aa8-b4a2-cd203eb6662a"
    );
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["id"], "1ea47840-aa39-5aa8-b4a2-cd203eb6662a");
    assert_eq!(
        body["stix_id"],
        "ipv4-addr--1ea47840-aa39-5aa8-b4a2-cd203eb6662a"
    );
}

#[tokio::test]
async fn it_returns_409_with_the_existing_id_for_another_spelling_of_the_same_network() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let first = test::TestRequest::post()
        .uri("/ipv4-networks")
        .set_json(json!({ "value": "203.0.113.0/24" }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, first).await;

    let second = test::TestRequest::post()
        .uri("/ipv4-networks")
        .set_json(json!({ "value": " 203.0.113.0/24" }))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, second).await;

    assert_eq!(resp.status(), 409);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(
        body,
        json!({
            "id": "a0895f10-d89d-56ad-a4a4-dc873ee08b8c",
            "stix_id": "ipv4-addr--a0895f10-d89d-56ad-a4a4-dc873ee08b8c"
        })
    );
}

#[tokio::test]
async fn it_returns_400_for_an_invalid_value() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/ipv4-networks")
        .set_json(json!({ "value": "198.51.100.7/24" }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn it_returns_400_for_a_value_of_another_kind() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/ipv4-networks")
        .set_json(json!({ "value": "198.51.100.7" }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn it_returns_400_when_the_client_tries_to_choose_the_id() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/ipv4-networks")
        .set_json(json!({
            "id": "550e8400-e29b-41d4-8716-446655440000",
            "value": "10.0.0.0/8"
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}
