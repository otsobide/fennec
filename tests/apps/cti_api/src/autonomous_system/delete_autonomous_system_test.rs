use actix_web::{dev::ServiceResponse, test, App};
use serde_json::json;

use cti_api::{build_state, configure_routes};

#[tokio::test]
async fn it_returns_204_and_forgets_the_autonomous_system() {
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

    let delete = test::TestRequest::delete()
        .uri("/autonomous-systems/7afc7bdf-d070-5add-90ff-f93503b1654d")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, delete).await;
    assert_eq!(resp.status(), 204);

    let find = test::TestRequest::get()
        .uri("/autonomous-systems/7afc7bdf-d070-5add-90ff-f93503b1654d")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, find).await;
    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn it_returns_404_for_an_autonomous_system_that_is_not_registered() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::delete()
        .uri("/autonomous-systems/0204c07d-e4dd-5f14-a3d5-c93cb1c5a9fc")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn it_returns_400_for_an_invalid_id() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::delete()
        .uri("/autonomous-systems/not-a-uuid")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn it_returns_404_when_deleting_twice() {
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

    let first = test::TestRequest::delete()
        .uri("/autonomous-systems/7afc7bdf-d070-5add-90ff-f93503b1654d")
        .to_request();
    let _: ServiceResponse = test::call_service(&app, first).await;

    let second = test::TestRequest::delete()
        .uri("/autonomous-systems/7afc7bdf-d070-5add-90ff-f93503b1654d")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, second).await;

    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn it_returns_400_for_a_uuid_v4_id() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::delete()
        .uri("/autonomous-systems/550e8400-e29b-41d4-8716-446655440000")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400);
}
