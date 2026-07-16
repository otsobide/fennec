use actix_web::{dev::ServiceResponse, test, App};
use serde_json::json;

use cti_api::{build_state, configure_routes};

#[tokio::test]
async fn it_returns_201_when_sighting_is_created() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/sightings")
        .set_json(json!({
            "id": "11111111-1111-1111-1111-111111111111",
            "ioc_id": "22222222-2222-2222-2222-222222222222",
            "source_id": "33333333-3333-3333-3333-333333333333",
            "observed_at": 1_700_000_000_u64,
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);
}

#[tokio::test]
async fn it_returns_409_when_creating_a_duplicate_id() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    // First create a sighting with (id_a, ioc_a, source_a).
    let first = test::TestRequest::post()
        .uri("/sightings")
        .set_json(json!({
            "id": "aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa",
            "ioc_id": "bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb",
            "source_id": "cccccccc-cccc-cccc-cccc-cccccccccccc",
            "observed_at": 1_700_000_000_u64,
        }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, first).await;

    // Now attempt to re-use the same id but with a different pair. The
    // in-memory repo prioritises pair collisions when the pair also matches;
    // here we use a fresh ioc/source pair so only the id collides.
    let second = test::TestRequest::post()
        .uri("/sightings")
        .set_json(json!({
            "id": "aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa",
            "ioc_id": "dddddddd-dddd-dddd-dddd-dddddddddddd",
            "source_id": "eeeeeeee-eeee-eeee-eeee-eeeeeeeeeeee",
            "observed_at": 1_700_000_000_u64,
        }))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, second).await;

    assert_eq!(resp.status(), 409);
}

#[tokio::test]
async fn it_returns_409_and_existing_id_when_pair_already_exists() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let existing_id = "12341234-1234-1234-1234-123412341234";
    let ioc_id = "aaaa1111-aaaa-1111-aaaa-1111aaaa1111";
    let source_id = "bbbb2222-bbbb-2222-bbbb-2222bbbb2222";

    let first = test::TestRequest::post()
        .uri("/sightings")
        .set_json(json!({
            "id": existing_id,
            "ioc_id": ioc_id,
            "source_id": source_id,
            "observed_at": 1_700_000_000_u64,
        }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, first).await;

    let second = test::TestRequest::post()
        .uri("/sightings")
        .set_json(json!({
            "id": "99999999-9999-9999-9999-999999999999",
            "ioc_id": ioc_id,
            "source_id": source_id,
            "observed_at": 1_700_000_001_u64,
        }))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, second).await;

    assert_eq!(resp.status(), 409);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["existing_id"], existing_id);
    assert!(body["message"].is_string());
}

#[tokio::test]
async fn it_returns_400_for_invalid_uuid() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/sightings")
        .set_json(json!({
            "id": "not-a-uuid",
            "ioc_id": "22222222-2222-2222-2222-222222222222",
            "source_id": "33333333-3333-3333-3333-333333333333",
            "observed_at": 1_700_000_000_u64,
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn it_returns_400_for_missing_body_fields() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/sightings")
        .set_json(json!({ "id": "11111111-1111-1111-1111-111111111111" }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}
