use actix_web::{dev::ServiceResponse, test, App};
use serde_json::json;

use cti_api::{build_state, configure_routes};

#[tokio::test]
async fn it_returns_all_sightings_of_the_given_observable() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let observable_id = "ffffffff-1111-4222-8333-444444444444";
    let source_a = "aaaaaaaa-1111-4111-8111-111111111111";
    let source_b = "bbbbbbbb-2222-4222-8222-222222222222";

    let post_a = test::TestRequest::post()
        .uri("/sightings")
        .set_json(json!({
            "id": "11111111-aaaa-4aaa-8aaa-aaaaaaaaaaaa",
            "observable_kind": "file",
            "observable_id": observable_id,
            "source_id": source_a,
            "observed_at": 1_700_000_000_u64,
        }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, post_a).await;

    let post_b = test::TestRequest::post()
        .uri("/sightings")
        .set_json(json!({
            "id": "22222222-bbbb-4bbb-8bbb-bbbbbbbbbbbb",
            "observable_kind": "file",
            "observable_id": observable_id,
            "source_id": source_b,
            "observed_at": 1_700_000_100_u64,
        }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, post_b).await;

    let list = test::TestRequest::get()
        .uri(&format!("/observables/file/{observable_id}/sightings"))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, list).await;
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    let arr = body.as_array().expect("response body should be an array");
    assert_eq!(arr.len(), 2);
    for item in arr {
        assert_eq!(item["observable_kind"], "file");
        assert_eq!(item["observable_id"], observable_id);
    }
}

#[tokio::test]
async fn it_returns_empty_array_when_the_observable_has_no_sightings() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/observables/file/deadbeef-dead-4eef-8ead-beefdeadbeef/sightings")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.as_array().unwrap().is_empty());
}

#[tokio::test]
async fn it_returns_400_when_the_observable_id_is_not_a_uuid() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/observables/file/not-a-uuid/sightings")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn it_returns_400_when_the_observable_kind_is_not_a_module_name() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/observables/Ipv4-Address/deadbeef-dead-4eef-8ead-beefdeadbeef/sightings")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn it_keeps_observables_of_different_kinds_apart() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let shared_id = "abfb438a-8baf-5f54-b19e-b85ee74b3ed7";
    let req = test::TestRequest::post()
        .uri("/sightings")
        .set_json(serde_json::json!({
            "id": "abababab-abab-4bab-8bab-abababababab",
            "observable_kind": "ipv4_address",
            "observable_id": shared_id,
            "source_id": "cdcdcdcd-cdcd-4dcd-8dcd-cdcdcdcdcdcd",
            "observed_at": 1700000000u64
        }))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);

    let other_kind = test::TestRequest::get()
        .uri(&format!("/observables/hostname/{shared_id}/sightings"))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, other_kind).await;
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.as_array().unwrap().is_empty());
}
