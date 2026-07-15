use actix_web::{dev::ServiceResponse, test, App};
use serde_json::json;

use cti_api::{build_state, configure_routes};

#[tokio::test]
async fn it_updates_an_existing_url_source() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let id = "c1c1c1c1-c1c1-c1c1-c1c1-c1c1c1c1c1c1";

    let post = test::TestRequest::post()
        .uri("/url-sources")
        .set_json(json!({
            "id": id,
            "url": "https://feeds.example.com/v1.txt",
            "format": "plain",
            "polling_interval_seconds": 3600
        }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, post).await;

    let put = test::TestRequest::put()
        .uri(&format!("/url-sources/{id}"))
        .set_json(json!({
            "url": "https://feeds.example.com/v2.json",
            "format": "json",
            "polling_interval_seconds": 60
        }))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, put).await;
    assert_eq!(resp.status(), 200);

    let get = test::TestRequest::get()
        .uri(&format!("/url-sources/{id}"))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, get).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["url"], "https://feeds.example.com/v2.json");
    assert_eq!(body["format"], "json");
    assert_eq!(body["polling_interval_seconds"], 60);
}

#[tokio::test]
async fn it_returns_404_when_updating_unknown_url_source() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::put()
        .uri("/url-sources/00000000-0000-0000-0000-000000000000")
        .set_json(json!({
            "url": "https://feeds.example.com/x.txt",
            "format": "plain",
            "polling_interval_seconds": 60
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn it_returns_400_for_invalid_url_on_update() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let id = "d2d2d2d2-d2d2-d2d2-d2d2-d2d2d2d2d2d2";

    let post = test::TestRequest::post()
        .uri("/url-sources")
        .set_json(json!({
            "id": id,
            "url": "https://feeds.example.com/x.txt",
            "format": "plain",
            "polling_interval_seconds": 3600
        }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, post).await;

    let put = test::TestRequest::put()
        .uri(&format!("/url-sources/{id}"))
        .set_json(json!({
            "url": "ftp://bad",
            "format": "plain",
            "polling_interval_seconds": 60
        }))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, put).await;
    assert_eq!(resp.status(), 400);
}
