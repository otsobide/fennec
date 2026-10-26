use actix_web::{dev::ServiceResponse, test, App};

use cti_api::{build_state, configure_routes};

use crate::src::dns_resolution::support::{
    record_ipv4, EXAMPLE_COM_TO_IPV4_ID, SOURCE_A, SOURCE_B,
};

#[tokio::test]
async fn it_returns_204_and_forgets_the_resolution_with_every_observation() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;
    for source_id in [SOURCE_A, SOURCE_B] {
        let req = record_ipv4(
            "example.com",
            "93.184.216.34",
            source_id,
            1_700_000_000,
            1_700_086_400,
        )
        .to_request();
        let _: ServiceResponse = test::call_service(&app, req).await;
    }

    let delete = test::TestRequest::delete()
        .uri("/dns-resolutions/2da44331-2011-5539-b65e-9ff44db9c139")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, delete).await;
    assert_eq!(resp.status(), 204);

    let find = test::TestRequest::get()
        .uri(&format!("/dns-resolutions/{EXAMPLE_COM_TO_IPV4_ID}"))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, find).await;
    assert_eq!(resp.status(), 404);

    let list = test::TestRequest::get()
        .uri("/dns-resolutions?ipv4_address=93.184.216.34")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, list).await;
    assert_eq!(body["dns_resolutions"], serde_json::json!([]));
}

#[tokio::test]
async fn it_returns_404_for_a_resolution_that_is_not_recorded() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::delete()
        .uri(&format!("/dns-resolutions/{EXAMPLE_COM_TO_IPV4_ID}"))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn it_returns_404_when_deleting_twice() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;
    let create = record_ipv4(
        "example.com",
        "93.184.216.34",
        SOURCE_A,
        1_700_000_000,
        1_700_086_400,
    )
    .to_request();
    let _: ServiceResponse = test::call_service(&app, create).await;
    let first = test::TestRequest::delete()
        .uri(&format!("/dns-resolutions/{EXAMPLE_COM_TO_IPV4_ID}"))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, first).await;

    let second = test::TestRequest::delete()
        .uri(&format!("/dns-resolutions/{EXAMPLE_COM_TO_IPV4_ID}"))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, second).await;

    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn it_returns_400_for_an_id_that_is_not_a_uuid() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::delete()
        .uri("/dns-resolutions/not-a-uuid")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400);
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
        .uri(&format!("/dns-resolutions/{SOURCE_A}"))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400);
}
