use actix_web::{dev::ServiceResponse, test, App};
use serde_json::json;

use cti_api::{build_state, configure_routes};

#[tokio::test]
async fn it_returns_all_sightings_for_the_given_ioc() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let ioc_id = "ffffffff-1111-4222-8333-444444444444";
    let source_a = "aaaaaaaa-1111-4111-8111-111111111111";
    let source_b = "bbbbbbbb-2222-4222-8222-222222222222";

    let post_a = test::TestRequest::post()
        .uri("/sightings")
        .set_json(json!({
            "id": "11111111-aaaa-4aaa-8aaa-aaaaaaaaaaaa",
            "ioc_id": ioc_id,
            "source_id": source_a,
            "observed_at": 1_700_000_000_u64,
        }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, post_a).await;

    let post_b = test::TestRequest::post()
        .uri("/sightings")
        .set_json(json!({
            "id": "22222222-bbbb-4bbb-8bbb-bbbbbbbbbbbb",
            "ioc_id": ioc_id,
            "source_id": source_b,
            "observed_at": 1_700_000_100_u64,
        }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, post_b).await;

    let list = test::TestRequest::get()
        .uri(&format!("/iocs/{ioc_id}/sightings"))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, list).await;
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    let arr = body.as_array().expect("response body should be an array");
    assert_eq!(arr.len(), 2);
    for item in arr {
        assert_eq!(item["ioc_id"], ioc_id);
    }
}

#[tokio::test]
async fn it_returns_empty_array_when_ioc_has_no_sightings() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/iocs/deadbeef-dead-4eef-8ead-beefdeadbeef/sightings")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.as_array().unwrap().is_empty());
}

#[tokio::test]
async fn it_returns_400_when_ioc_id_is_not_a_uuid() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/iocs/not-a-uuid/sightings")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}
