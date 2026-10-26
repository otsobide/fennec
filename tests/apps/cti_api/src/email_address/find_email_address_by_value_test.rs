use actix_web::{dev::ServiceResponse, test, App};
use serde_json::json;

use cti_api::{build_state, configure_routes};

#[tokio::test]
async fn it_finds_an_address_by_any_spelling_of_its_value() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let create = test::TestRequest::post()
        .uri("/email-addresses")
        .set_json(json!({ "value": "john@example.com" }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, create).await;

    let req = test::TestRequest::get()
        .uri("/email-addresses?value=John%40Example.COM")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["id"], "7165e2a9-671f-585d-b1e1-ca59c671d934");
    assert_eq!(body["value"], "john@example.com");
}

#[tokio::test]
async fn it_returns_404_for_a_value_that_is_not_registered() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/email-addresses?value=billing@xn--bcher-kva.example")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn it_returns_400_for_an_invalid_value() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/email-addresses?value=john@@example.com")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400);
}
