use actix_web::{dev::ServiceResponse, test, App};
use serde_json::json;

use cti_api::{build_state, configure_routes};

#[tokio::test]
async fn it_returns_200_with_the_hostname() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let create = test::TestRequest::post()
        .uri("/hostnames")
        .set_json(json!({ "value": "mail01.corp.example" }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, create).await;

    let req = test::TestRequest::get()
        .uri("/hostnames/38ee45a4-70cb-5ddf-9cd2-abcd7afa92a2")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["id"], "38ee45a4-70cb-5ddf-9cd2-abcd7afa92a2");
    assert_eq!(
        body["stix_id"],
        "hostname--38ee45a4-70cb-5ddf-9cd2-abcd7afa92a2"
    );
    assert_eq!(body["value"], "mail01.corp.example");
    assert!(body["created_at"].as_u64().unwrap() > 0);
}

#[tokio::test]
async fn it_returns_404_for_a_hostname_that_is_not_registered() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/hostnames/46e518b9-291e-5e2e-b34c-18e18439c5d1")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn it_returns_400_for_an_id_that_is_not_a_uuid_v5() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/hostnames/550e8400-e29b-41d4-8716-446655440000")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400);
}
