use actix_web::{dev::ServiceResponse, test, App};
use serde_json::json;

use cti_api::{build_state, configure_routes};

#[tokio::test]
async fn it_returns_200_with_the_ipv6_address() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let create = test::TestRequest::post()
        .uri("/ipv6-addresses")
        .set_json(json!({ "value": "2001:db8:85a3::8a2e:370:7334" }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, create).await;

    let req = test::TestRequest::get()
        .uri("/ipv6-addresses/f5344921-72d2-5619-945d-8757bca4b7cd")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["id"], "f5344921-72d2-5619-945d-8757bca4b7cd");
    assert_eq!(
        body["stix_id"],
        "ipv6-addr--f5344921-72d2-5619-945d-8757bca4b7cd"
    );
    assert_eq!(body["value"], "2001:db8:85a3::8a2e:370:7334");
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
        .uri("/ipv6-addresses/9ed76cd5-ab3b-55a5-b6ec-b55c3efb0be2")
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
        .uri("/ipv6-addresses/550e8400-e29b-41d4-8716-446655440000")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400);
}
