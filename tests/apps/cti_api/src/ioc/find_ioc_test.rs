use actix_web::{dev::ServiceResponse, test, App};
use serde_json::json;

use cti_api::{build_state, configure_routes};

#[tokio::test]
async fn it_persists_the_ioc_and_can_be_retrieved() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let id = "a1b2c3d4-e5f6-7890-abcd-ef1234567890";

    let post_req = test::TestRequest::post()
        .uri("/iocs")
        .set_json(json!({
            "id": id,
            "ioc_type": "sha256",
            "value": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, post_req).await;

    let get_req = test::TestRequest::get()
        .uri(&format!("/iocs/{id}"))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, get_req).await;

    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["id"], id);
    assert_eq!(body["ioc_type"], "sha256");
    assert_eq!(
        body["value"],
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
    assert!(body["created_at"].is_number());
    assert!(body["updated_at"].is_number());
}

#[tokio::test]
async fn it_returns_404_when_ioc_does_not_exist() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/iocs/00000000-0000-0000-0000-000000000000")
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn it_returns_400_when_path_id_is_not_a_uuid() {
    let app = test::init_service(
        App::new().app_data(build_state()).configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::get().uri("/iocs/not-a-uuid").to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}
