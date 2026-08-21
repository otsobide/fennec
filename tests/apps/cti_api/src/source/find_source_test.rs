use actix_web::{dev::ServiceResponse, test, App};
use serde_json::json;

use cti_api::{build_state, configure_routes};

#[tokio::test]
async fn it_persists_the_source_and_can_be_retrieved() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let id = "a1b2c3d4-e5f6-4890-8bcd-ef1234567890";

    let post_req = test::TestRequest::post()
        .uri("/sources")
        .set_json(json!({
            "id": id,
            "source_type": "url",
            "status": "active",
            "description": "my source"
        }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, post_req).await;

    let get_req = test::TestRequest::get()
        .uri(&format!("/sources/{id}"))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, get_req).await;

    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["id"], id);
    assert_eq!(body["source_type"], "url");
    assert_eq!(body["status"], "active");
    assert_eq!(body["description"], "my source");
    assert!(body["created_at"].is_number());
    assert!(body["updated_at"].is_number());
}

#[tokio::test]
async fn it_returns_404_when_source_does_not_exist() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/sources/00000000-0000-4000-8000-000000000000")
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
        .uri("/sources/not-a-uuid")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}
