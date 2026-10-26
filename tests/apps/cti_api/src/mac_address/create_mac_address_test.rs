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
        .uri("/mac-addresses")
        .set_json(json!({ "value": "d2:fb:49:24:37:18" }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);
    assert_eq!(
        resp.headers().get("Location").unwrap(),
        "/mac-addresses/757b1725-9903-54f5-a855-1240691d7659"
    );
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["id"], "757b1725-9903-54f5-a855-1240691d7659");
    assert_eq!(
        body["stix_id"],
        "mac-addr--757b1725-9903-54f5-a855-1240691d7659"
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
        .uri("/mac-addresses")
        .set_json(json!({ "value": "00:1b:63:84:45:e6" }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, first).await;

    let second = test::TestRequest::post()
        .uri("/mac-addresses")
        .set_json(json!({ "value": "001B.6384.45E6" }))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, second).await;

    assert_eq!(resp.status(), 409);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(
        body,
        json!({
            "id": "46e4e100-1877-5fe6-a06f-c77917889a2b",
            "stix_id": "mac-addr--46e4e100-1877-5fe6-a06f-c77917889a2b"
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
        .uri("/mac-addresses")
        .set_json(json!({ "value": "d2:fb:49:24:37" }))
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
        .uri("/mac-addresses")
        .set_json(json!({ "value": "192.0.2.1" }))
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
        .uri("/mac-addresses")
        .set_json(json!({
            "id": "550e8400-e29b-41d4-8716-446655440000",
            "value": "02:00:5e:10:00:00"
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}
