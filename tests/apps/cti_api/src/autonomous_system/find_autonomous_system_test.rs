use actix_web::{dev::ServiceResponse, test, App};
use serde_json::json;

use cti_api::{build_state, configure_routes};

#[tokio::test]
async fn it_returns_200_with_the_autonomous_system() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let create = test::TestRequest::post()
        .uri("/autonomous-systems")
        .set_json(json!({ "value": "15169" }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, create).await;

    let req = test::TestRequest::get()
        .uri("/autonomous-systems/7afc7bdf-d070-5add-90ff-f93503b1654d")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["id"], "7afc7bdf-d070-5add-90ff-f93503b1654d");
    assert_eq!(
        body["stix_id"],
        "autonomous-system--7afc7bdf-d070-5add-90ff-f93503b1654d"
    );
    assert_eq!(body["value"], "15169");
    assert!(body["created_at"].as_u64().unwrap() > 0);
}

#[tokio::test]
async fn it_returns_404_for_an_autonomous_system_that_is_not_registered() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/autonomous-systems/0204c07d-e4dd-5f14-a3d5-c93cb1c5a9fc")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn it_returns_400_for_an_id_that_is_not_a_uuid_v5() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/autonomous-systems/550e8400-e29b-41d4-8716-446655440000")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400);
}
