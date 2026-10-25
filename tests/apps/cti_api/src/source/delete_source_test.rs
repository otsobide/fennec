use actix_web::{dev::ServiceResponse, test, App};
use serde_json::json;

use cti_api::{build_state, configure_routes};

#[tokio::test]
async fn it_deletes_an_existing_source() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let id = "e3e3e3e3-e3e3-43e3-83e3-e3e3e3e3e3e3";

    let post = test::TestRequest::post()
        .uri("/sources")
        .set_json(json!({
            "id": id,
            "source_type": "url",
            "status": "active",
            "description": "to delete"
        }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, post).await;

    let delete = test::TestRequest::delete()
        .uri(&format!("/sources/{id}"))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, delete).await;
    assert_eq!(resp.status(), 204);

    let get = test::TestRequest::get()
        .uri(&format!("/sources/{id}"))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, get).await;
    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn it_returns_404_when_deleting_unknown_source() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::delete()
        .uri("/sources/00000000-0000-4000-8000-000000000000")
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}
