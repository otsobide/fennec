use actix_web::{dev::ServiceResponse, test, App};
use serde_json::json;

use cti_api::{build_state, configure_routes};

#[tokio::test]
async fn it_bumps_count_and_updates_last_seen() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let id = "11112222-3333-4444-8555-666677778888";
    let first_ts: u64 = 1_700_000_000;
    let second_ts: u64 = 1_700_000_500;

    let post = test::TestRequest::post()
        .uri("/sightings")
        .set_json(json!({
            "id": id,
            "ioc_id": "aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee",
            "source_id": "11111111-2222-4333-8444-555555555555",
            "observed_at": first_ts,
        }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, post).await;

    let observe = test::TestRequest::post()
        .uri(&format!("/sightings/{id}/observations"))
        .set_json(json!({ "observed_at": second_ts }))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, observe).await;
    assert_eq!(resp.status(), 200);

    let get = test::TestRequest::get()
        .uri(&format!("/sightings/{id}"))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, get).await;
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["count"], 2);
    assert_eq!(body["last_seen"], second_ts);
    assert_eq!(body["first_seen"], first_ts);
}

#[tokio::test]
async fn it_returns_404_when_observing_unknown_sighting() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/sightings/00000000-0000-4000-8000-000000000000/observations")
        .set_json(json!({ "observed_at": 1_700_000_000_u64 }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}
