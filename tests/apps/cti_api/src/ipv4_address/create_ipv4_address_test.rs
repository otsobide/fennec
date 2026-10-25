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
        .uri("/ipv4-addresses")
        .set_json(json!({ "value": "89.19.209.154" }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);
    assert_eq!(
        resp.headers().get("Location").unwrap(),
        "/ipv4-addresses/abfb438a-8baf-5f54-b19e-b85ee74b3ed7"
    );
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["id"], "abfb438a-8baf-5f54-b19e-b85ee74b3ed7");
    assert_eq!(
        body["stix_id"],
        "ipv4-addr--abfb438a-8baf-5f54-b19e-b85ee74b3ed7"
    );
}

#[tokio::test]
async fn it_returns_409_with_the_existing_id_for_another_spelling_of_the_same_address() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let first = test::TestRequest::post()
        .uri("/ipv4-addresses")
        .set_json(json!({ "value": "198.51.100.3" }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, first).await;

    let second = test::TestRequest::post()
        .uri("/ipv4-addresses")
        .set_json(json!({ "value": " 198.51.100.3/32 " }))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, second).await;

    assert_eq!(resp.status(), 409);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(
        body,
        json!({
            "id": "28bb3599-77cd-5a82-a950-b5bc3caf07c4",
            "stix_id": "ipv4-addr--28bb3599-77cd-5a82-a950-b5bc3caf07c4"
        })
    );
}

#[tokio::test]
async fn it_returns_400_for_a_network() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/ipv4-addresses")
        .set_json(json!({ "value": "198.51.100.0/24" }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn it_returns_400_for_a_value_that_is_not_an_ipv4_address() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/ipv4-addresses")
        .set_json(json!({ "value": "malicious.example.com" }))
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
        .uri("/ipv4-addresses")
        .set_json(json!({
            "id": "550e8400-e29b-41d4-8716-446655440000",
            "value": "198.51.100.7"
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}
