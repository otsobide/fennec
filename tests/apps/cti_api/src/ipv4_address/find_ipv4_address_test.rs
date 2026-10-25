use actix_web::{dev::ServiceResponse, test, App};
use serde_json::json;

use cti_api::{build_state, configure_routes};

#[tokio::test]
async fn it_returns_200_with_the_ipv4_address() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let create = test::TestRequest::post()
        .uri("/ipv4-addresses")
        .set_json(json!({ "value": "89.19.209.154" }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, create).await;

    let req = test::TestRequest::get()
        .uri("/ipv4-addresses/abfb438a-8baf-5f54-b19e-b85ee74b3ed7")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["id"], "abfb438a-8baf-5f54-b19e-b85ee74b3ed7");
    assert_eq!(
        body["stix_id"],
        "ipv4-addr--abfb438a-8baf-5f54-b19e-b85ee74b3ed7"
    );
    assert_eq!(body["value"], "89.19.209.154");
    assert!(body["created_at"].as_u64().unwrap() > 0);
}

#[tokio::test]
async fn it_returns_404_for_an_address_that_is_not_registered() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/ipv4-addresses/28bb3599-77cd-5a82-a950-b5bc3caf07c4")
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
        .uri("/ipv4-addresses/550e8400-e29b-41d4-8716-446655440000")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400);
}
