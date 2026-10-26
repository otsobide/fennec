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
        .uri("/domain-names")
        .set_json(json!({ "value": "hostelhotels.net" }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);
    assert_eq!(
        resp.headers().get("Location").unwrap(),
        "/domain-names/07f3ac51-335f-524c-918f-86f80da2bce3"
    );
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["id"], "07f3ac51-335f-524c-918f-86f80da2bce3");
    assert_eq!(
        body["stix_id"],
        "domain-name--07f3ac51-335f-524c-918f-86f80da2bce3"
    );
}

#[tokio::test]
async fn it_returns_409_with_the_existing_id_for_another_spelling_of_the_same_name() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let first = test::TestRequest::post()
        .uri("/domain-names")
        .set_json(json!({ "value": "example.com" }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, first).await;

    let second = test::TestRequest::post()
        .uri("/domain-names")
        .set_json(json!({ "value": "EXAMPLE.com." }))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, second).await;

    assert_eq!(resp.status(), 409);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(
        body,
        json!({
            "id": "bedb4899-d24b-5401-bc86-8f6b4cc18ec7",
            "stix_id": "domain-name--bedb4899-d24b-5401-bc86-8f6b4cc18ec7"
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
        .uri("/domain-names")
        .set_json(json!({ "value": "example..com" }))
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
        .uri("/domain-names")
        .set_json(json!({ "value": "203.0.113.9" }))
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
        .uri("/domain-names")
        .set_json(json!({
            "id": "550e8400-e29b-41d4-8716-446655440000",
            "value": "update.example-cdn.net"
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}
