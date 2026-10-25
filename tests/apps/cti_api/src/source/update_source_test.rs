use actix_web::{dev::ServiceResponse, test, App};
use serde_json::json;

use cti_api::{build_state, configure_routes};

#[tokio::test]
async fn it_updates_an_existing_source() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let id = "c1c1c1c1-c1c1-41c1-81c1-c1c1c1c1c1c1";

    let post = test::TestRequest::post()
        .uri("/sources")
        .set_json(json!({
            "id": id,
            "source_type": "url",
            "status": "active",
            "description": "before"
        }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, post).await;

    let put = test::TestRequest::put()
        .uri(&format!("/sources/{id}"))
        .set_json(json!({ "status": "inactive", "description": "after" }))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, put).await;
    assert_eq!(resp.status(), 200);

    let get = test::TestRequest::get()
        .uri(&format!("/sources/{id}"))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, get).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["status"], "inactive");
    assert_eq!(body["description"], "after");
}

#[tokio::test]
async fn it_returns_404_when_updating_unknown_source() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::put()
        .uri("/sources/00000000-0000-4000-8000-000000000000")
        .set_json(json!({ "status": "inactive", "description": "x" }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn it_returns_400_for_unknown_status_on_update() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let id = "d2d2d2d2-d2d2-42d2-82d2-d2d2d2d2d2d2";

    let post = test::TestRequest::post()
        .uri("/sources")
        .set_json(json!({
            "id": id,
            "source_type": "url",
            "status": "active",
            "description": "x"
        }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, post).await;

    let put = test::TestRequest::put()
        .uri(&format!("/sources/{id}"))
        .set_json(json!({ "status": "paused", "description": "y" }))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, put).await;
    assert_eq!(resp.status(), 400);
}
