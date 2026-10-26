use actix_web::{dev::ServiceResponse, test, App};
use serde_json::{json, Value};

use cti_api::{build_state, configure_routes};

const GOOGLE: &str = "/autonomous-systems/7afc7bdf-d070-5add-90ff-f93503b1654d";

macro_rules! app {
    () => {
        test::init_service(
            App::new()
                .app_data(build_state())
                .configure(configure_routes),
        )
        .await
    };
}

macro_rules! register_google {
    ($app:expr) => {{
        let create = test::TestRequest::post()
            .uri("/autonomous-systems")
            .set_json(json!({ "value": "AS15169" }))
            .to_request();
        let resp: ServiceResponse = test::call_service(&$app, create).await;
        assert_eq!(resp.status(), 201);
    }};
}

#[tokio::test]
async fn it_starts_without_a_name_or_registry() {
    let app = app!();
    register_google!(app);

    let find = test::TestRequest::get().uri(GOOGLE).to_request();
    let body: Value = test::call_and_read_body_json(&app, find).await;

    assert_eq!(body["name"], Value::Null);
    assert_eq!(body["rir"], Value::Null);
    assert_eq!(body["updated_at"], body["created_at"]);
}

#[tokio::test]
async fn it_returns_200_and_describes_the_autonomous_system() {
    let app = app!();
    register_google!(app);

    let update = test::TestRequest::put()
        .uri(GOOGLE)
        .set_json(json!({ "name": " GOOGLE ", "rir": "arin" }))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, update).await;
    assert_eq!(resp.status(), 200);

    let find = test::TestRequest::get().uri(GOOGLE).to_request();
    let body: Value = test::call_and_read_body_json(&app, find).await;
    assert_eq!(body["value"], "15169");
    assert_eq!(body["name"], "GOOGLE");
    assert_eq!(body["rir"], "ARIN");
    assert!(body["updated_at"].as_u64() >= body["created_at"].as_u64());
}

#[tokio::test]
async fn it_clears_fields_that_are_null_or_missing() {
    let app = app!();
    register_google!(app);
    let describe = test::TestRequest::put()
        .uri(GOOGLE)
        .set_json(json!({ "name": "GOOGLE", "rir": "ARIN" }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, describe).await;

    let clear = test::TestRequest::put()
        .uri(GOOGLE)
        .set_json(json!({ "name": null }))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, clear).await;
    assert_eq!(resp.status(), 200);

    let find = test::TestRequest::get().uri(GOOGLE).to_request();
    let body: Value = test::call_and_read_body_json(&app, find).await;
    assert_eq!(body["name"], Value::Null);
    assert_eq!(body["rir"], Value::Null);
}

#[tokio::test]
async fn it_returns_404_for_an_autonomous_system_that_is_not_registered() {
    let app = app!();

    let update = test::TestRequest::put()
        .uri(GOOGLE)
        .set_json(json!({ "name": "GOOGLE" }))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, update).await;

    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn it_returns_400_for_an_invalid_name_or_registry() {
    let app = app!();
    register_google!(app);

    for body in [
        json!({ "name": "GOOGLE\nARIN" }),
        json!({ "name": "   " }),
        json!({ "rir": "IANA" }),
    ] {
        let update = test::TestRequest::put()
            .uri(GOOGLE)
            .set_json(body.clone())
            .to_request();
        let resp: ServiceResponse = test::call_service(&app, update).await;
        assert_eq!(resp.status(), 400, "{body}");
    }
}

#[tokio::test]
async fn it_returns_400_when_the_client_tries_to_change_the_number() {
    let app = app!();
    register_google!(app);

    let update = test::TestRequest::put()
        .uri(GOOGLE)
        .set_json(json!({ "value": "13335", "name": "CLOUDFLARENET" }))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, update).await;

    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn it_returns_400_for_an_id_that_is_not_a_uuid_v5() {
    let app = app!();

    let update = test::TestRequest::put()
        .uri("/autonomous-systems/0b1a7f2e-8a45-4d2c-9f5e-6a7b8c9d0e1f")
        .set_json(json!({ "name": "GOOGLE" }))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, update).await;

    assert_eq!(resp.status(), 400);
}
