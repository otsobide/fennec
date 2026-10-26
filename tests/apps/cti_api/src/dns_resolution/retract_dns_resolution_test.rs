use actix_web::{dev::ServiceResponse, test, App};

use cti_api::{build_state, configure_routes};

use crate::src::dns_resolution::support::{
    record_ipv4, EXAMPLE_COM_TO_IPV4_ID, SOURCE_A, SOURCE_B,
};

#[tokio::test]
async fn it_returns_204_and_keeps_the_resolution_while_another_source_asserts_it() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;
    for (source_id, first_seen, last_seen) in [
        (SOURCE_A, 1_700_000_000, 1_700_086_400),
        (SOURCE_B, 1_710_000_000, 1_710_086_400),
    ] {
        let req = record_ipv4(
            "example.com",
            "93.184.216.34",
            source_id,
            first_seen,
            last_seen,
        )
        .to_request();
        let _: ServiceResponse = test::call_service(&app, req).await;
    }

    let retract = test::TestRequest::delete()
        .uri(&format!(
            "/dns-resolutions/{EXAMPLE_COM_TO_IPV4_ID}/observations/{SOURCE_B}"
        ))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, retract).await;
    assert_eq!(resp.status(), 204);

    let find = test::TestRequest::get()
        .uri(&format!("/dns-resolutions/{EXAMPLE_COM_TO_IPV4_ID}"))
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, find).await;
    let observations = body["observations"].as_array().unwrap();
    assert_eq!(observations.len(), 1);
    assert_eq!(observations[0]["source_id"], SOURCE_A);
    assert_eq!(body["first_seen"], 1_700_000_000u64);
    assert_eq!(body["last_seen"], 1_700_086_400u64);
}

#[tokio::test]
async fn it_returns_204_and_deletes_the_resolution_when_the_last_observation_is_retracted() {
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

    let retract = test::TestRequest::delete()
        .uri(&format!(
            "/dns-resolutions/{EXAMPLE_COM_TO_IPV4_ID}/observations/{SOURCE_A}"
        ))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, retract).await;
    assert_eq!(resp.status(), 204);

    let find = test::TestRequest::get()
        .uri(&format!("/dns-resolutions/{EXAMPLE_COM_TO_IPV4_ID}"))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, find).await;
    assert_eq!(resp.status(), 404);

    let list = test::TestRequest::get()
        .uri("/dns-resolutions?domain_name=example.com")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, list).await;
    assert_eq!(body["dns_resolutions"], serde_json::json!([]));
}

#[tokio::test]
async fn it_records_the_resolution_again_after_its_last_observation_was_retracted() {
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
    let retract = test::TestRequest::delete()
        .uri(&format!(
            "/dns-resolutions/{EXAMPLE_COM_TO_IPV4_ID}/observations/{SOURCE_A}"
        ))
        .to_request();
    let _: ServiceResponse = test::call_service(&app, retract).await;

    let again = record_ipv4(
        "example.com",
        "93.184.216.34",
        SOURCE_A,
        1_700_000_000,
        1_700_086_400,
    )
    .to_request();
    let resp: ServiceResponse = test::call_service(&app, again).await;

    assert_eq!(resp.status(), 201);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["outcome"], "created");
}

#[tokio::test]
async fn it_returns_404_for_a_source_that_has_not_observed_the_resolution() {
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

    let retract = test::TestRequest::delete()
        .uri(&format!(
            "/dns-resolutions/{EXAMPLE_COM_TO_IPV4_ID}/observations/{SOURCE_B}"
        ))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, retract).await;
    assert_eq!(resp.status(), 404);

    let find = test::TestRequest::get()
        .uri(&format!("/dns-resolutions/{EXAMPLE_COM_TO_IPV4_ID}"))
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, find).await;
    assert_eq!(body["observations"].as_array().unwrap().len(), 1);
}

#[tokio::test]
async fn it_returns_404_for_a_resolution_that_is_not_recorded() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let retract = test::TestRequest::delete()
        .uri(&format!(
            "/dns-resolutions/{EXAMPLE_COM_TO_IPV4_ID}/observations/{SOURCE_A}"
        ))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, retract).await;

    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn it_returns_404_when_retracting_twice() {
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
    let uri = format!("/dns-resolutions/{EXAMPLE_COM_TO_IPV4_ID}/observations/{SOURCE_A}");
    let first = test::TestRequest::delete().uri(&uri).to_request();
    let _: ServiceResponse = test::call_service(&app, first).await;

    let second = test::TestRequest::delete().uri(&uri).to_request();
    let resp: ServiceResponse = test::call_service(&app, second).await;

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

    for id in ["not-a-uuid", SOURCE_B] {
        let retract = test::TestRequest::delete()
            .uri(&format!("/dns-resolutions/{id}/observations/{SOURCE_A}"))
            .to_request();
        let resp: ServiceResponse = test::call_service(&app, retract).await;
        assert_eq!(resp.status(), 400, "{id}");
    }
}

#[tokio::test]
async fn it_returns_400_for_a_source_id_that_is_not_a_uuid_v4() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    for source_id in ["not-a-uuid", EXAMPLE_COM_TO_IPV4_ID] {
        let retract = test::TestRequest::delete()
            .uri(&format!(
                "/dns-resolutions/{EXAMPLE_COM_TO_IPV4_ID}/observations/{source_id}"
            ))
            .to_request();
        let resp: ServiceResponse = test::call_service(&app, retract).await;
        assert_eq!(resp.status(), 400, "{source_id}");
    }
}
