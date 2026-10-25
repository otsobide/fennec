use actix_web::{dev::ServiceResponse, test, App};
use serde_json::json;

use cti_api::{build_state, configure_routes};

#[tokio::test]
async fn it_persists_the_sighting_and_can_be_retrieved() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let id = "01234567-89ab-4def-8123-456789abcdef";
    let ioc_id = "fedcba98-7654-4210-8edc-ba9876543210";
    let source_id = "11223344-5566-4788-89aa-bbccddeeff00";
    let observed_at: u64 = 1_700_000_123;

    let post_req = test::TestRequest::post()
        .uri("/sightings")
        .set_json(json!({
            "id": id,
            "ioc_id": ioc_id,
            "source_id": source_id,
            "observed_at": observed_at,
        }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, post_req).await;

    let get_req = test::TestRequest::get()
        .uri(&format!("/sightings/{id}"))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, get_req).await;

    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["id"], id);
    assert_eq!(body["ioc_id"], ioc_id);
    assert_eq!(body["source_id"], source_id);
    assert_eq!(body["count"], 1);
    assert_eq!(body["first_seen"], observed_at);
    assert_eq!(body["last_seen"], observed_at);
    assert!(body["created_at"].is_number());
    assert!(body["updated_at"].is_number());
}

#[tokio::test]
async fn it_returns_404_when_sighting_does_not_exist() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/sightings/00000000-0000-4000-8000-000000000000")
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn it_returns_400_when_path_id_is_not_a_uuid() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/sightings/not-a-uuid")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}
