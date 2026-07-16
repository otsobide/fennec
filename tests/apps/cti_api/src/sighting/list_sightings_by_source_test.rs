use actix_web::{dev::ServiceResponse, test, App};
use serde_json::json;

use cti_api::{build_state, configure_routes};

#[tokio::test]
async fn it_returns_all_sightings_for_the_given_source() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let source_id = "77777777-7777-7777-7777-777777777777";
    let ioc_a = "aaaaaaaa-3333-3333-3333-333333333333";
    let ioc_b = "bbbbbbbb-4444-4444-4444-444444444444";

    let post_a = test::TestRequest::post()
        .uri("/sightings")
        .set_json(json!({
            "id": "33333333-cccc-cccc-cccc-cccccccccccc",
            "ioc_id": ioc_a,
            "source_id": source_id,
            "observed_at": 1_700_000_000_u64,
        }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, post_a).await;

    let post_b = test::TestRequest::post()
        .uri("/sightings")
        .set_json(json!({
            "id": "44444444-dddd-dddd-dddd-dddddddddddd",
            "ioc_id": ioc_b,
            "source_id": source_id,
            "observed_at": 1_700_000_100_u64,
        }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, post_b).await;

    let list = test::TestRequest::get()
        .uri(&format!("/sources/{source_id}/sightings"))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, list).await;
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    let arr = body.as_array().expect("response body should be an array");
    assert_eq!(arr.len(), 2);
    for item in arr {
        assert_eq!(item["source_id"], source_id);
    }
}

#[tokio::test]
async fn it_returns_empty_array_when_source_has_no_sightings() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/sources/cafecafe-cafe-cafe-cafe-cafecafecafe/sightings")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.as_array().unwrap().is_empty());
}

#[tokio::test]
async fn it_returns_400_when_source_id_is_not_a_uuid() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/sources/not-a-uuid/sightings")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}
