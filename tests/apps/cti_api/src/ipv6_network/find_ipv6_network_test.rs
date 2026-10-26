use actix_web::{dev::ServiceResponse, test, App};
use serde_json::json;

use cti_api::{build_state, configure_routes};

#[tokio::test]
async fn it_returns_200_with_the_ipv6_network() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let create = test::TestRequest::post()
        .uri("/ipv6-networks")
        .set_json(json!({ "value": "2001:db8::/32" }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, create).await;

    let req = test::TestRequest::get()
        .uri("/ipv6-networks/8da31feb-e0f9-54c3-ab30-bd8ccea361dd")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["id"], "8da31feb-e0f9-54c3-ab30-bd8ccea361dd");
    assert_eq!(
        body["stix_id"],
        "ipv6-addr--8da31feb-e0f9-54c3-ab30-bd8ccea361dd"
    );
    assert_eq!(body["value"], "2001:db8::/32");
    assert!(body["created_at"].as_u64().unwrap() > 0);
}

#[tokio::test]
async fn it_returns_404_for_a_network_that_is_not_registered() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/ipv6-networks/2a50470b-c9dc-5b93-8fa0-d2c92c80e12e")
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
        .uri("/ipv6-networks/550e8400-e29b-41d4-8716-446655440000")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400);
}
