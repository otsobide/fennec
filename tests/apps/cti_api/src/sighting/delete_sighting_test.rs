use actix_web::{dev::ServiceResponse, test, App};
use serde_json::json;

use cti_api::{build_state, configure_routes};

#[tokio::test]
async fn it_deletes_an_existing_sighting() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let id = "abababab-abab-4bab-8bab-abababababab";

    let post = test::TestRequest::post()
        .uri("/sightings")
        .set_json(json!({
            "id": id,
            "ioc_id": "cdcdcdcd-cdcd-4dcd-8dcd-cdcdcdcdcdcd",
            "source_id": "efefefef-efef-4fef-8fef-efefefefefef",
            "observed_at": 1_700_000_000_u64,
        }))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, post).await;

    let delete = test::TestRequest::delete()
        .uri(&format!("/sightings/{id}"))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, delete).await;
    assert_eq!(resp.status(), 204);

    let get = test::TestRequest::get()
        .uri(&format!("/sightings/{id}"))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, get).await;
    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn it_returns_404_when_deleting_unknown_sighting() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::delete()
        .uri("/sightings/00000000-0000-4000-8000-000000000000")
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}
