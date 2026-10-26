use actix_web::{dev::ServiceResponse, test, App};
use serde_json::json;

use cti_api::{build_state, configure_routes};

#[tokio::test]
async fn it_returns_204_and_forgets_the_network() {
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

    let delete = test::TestRequest::delete()
        .uri("/ipv6-networks/8da31feb-e0f9-54c3-ab30-bd8ccea361dd")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, delete).await;
    assert_eq!(resp.status(), 204);

    let find = test::TestRequest::get()
        .uri("/ipv6-networks/8da31feb-e0f9-54c3-ab30-bd8ccea361dd")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, find).await;
    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn it_returns_404_for_a_network_that_is_not_registered() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::delete()
        .uri("/ipv6-networks/2a50470b-c9dc-5b93-8fa0-d2c92c80e12e")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn it_returns_400_for_an_invalid_id() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::delete()
        .uri("/ipv6-networks/not-a-uuid")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn it_returns_404_when_deleting_twice() {
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

    let first = test::TestRequest::delete()
        .uri("/ipv6-networks/8da31feb-e0f9-54c3-ab30-bd8ccea361dd")
        .to_request();
    let _: ServiceResponse = test::call_service(&app, first).await;

    let second = test::TestRequest::delete()
        .uri("/ipv6-networks/8da31feb-e0f9-54c3-ab30-bd8ccea361dd")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, second).await;

    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn it_returns_400_for_a_uuid_v4_id() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::delete()
        .uri("/ipv6-networks/550e8400-e29b-41d4-8716-446655440000")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400);
}
