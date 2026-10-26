use actix_web::{dev::ServiceResponse, test, App};
use serde_json::json;

use cti_api::{build_state, configure_routes};

use crate::src::dns_resolution::support::{
    record, record_ipv4, EXAMPLE_COM_ID, EXAMPLE_COM_TO_IPV4_ID, EXAMPLE_COM_TO_IPV4_STIX_ID,
    EXAMPLE_COM_TO_IPV6_ID, EXAMPLE_COM_TO_IPV6_STIX_ID, IPV4_ID, IPV6_ID, SOURCE_A, SOURCE_B,
};

#[tokio::test]
async fn it_returns_200_with_the_endpoints_the_window_and_the_observation() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;
    let create = record(json!({
        "domain_name": "example.com",
        "ipv4_address": "93.184.216.34",
        "source_id": SOURCE_A,
        "first_seen": 1_700_000_000u64,
        "last_seen": 1_700_086_400u64,
        "count": 42u64
    }))
    .to_request();
    let _: ServiceResponse = test::call_service(&app, create).await;

    let req = test::TestRequest::get()
        .uri("/dns-resolutions/2da44331-2011-5539-b65e-9ff44db9c139")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["id"], EXAMPLE_COM_TO_IPV4_ID);
    assert_eq!(body["stix_id"], EXAMPLE_COM_TO_IPV4_STIX_ID);
    assert_eq!(body["relationship_type"], "resolves-to");
    assert_eq!(
        body["subject"],
        json!({ "kind": "domain_name", "value": "example.com", "id": EXAMPLE_COM_ID })
    );
    assert_eq!(
        body["answer"],
        json!({ "kind": "ipv4_address", "value": "93.184.216.34", "id": IPV4_ID })
    );
    assert_eq!(body["first_seen"], 1_700_000_000u64);
    assert_eq!(body["last_seen"], 1_700_086_400u64);
    assert!(body["created_at"].as_u64().unwrap() > 1_700_000_000);

    let observations = body["observations"].as_array().unwrap();
    assert_eq!(observations.len(), 1);
    let observation = &observations[0];
    assert_eq!(observation["source_id"], SOURCE_A);
    assert_eq!(observation["first_seen"], 1_700_000_000u64);
    assert_eq!(observation["last_seen"], 1_700_086_400u64);
    assert_eq!(observation["count"], 42u64);
    assert_eq!(observation["created_at"], body["created_at"]);
    assert_eq!(observation["updated_at"], body["created_at"]);
    assert_eq!(observation.as_object().unwrap().len(), 6, "{observation}");
    assert_eq!(body.as_object().unwrap().len(), 9, "{body}");
}

#[tokio::test]
async fn it_counts_one_for_a_source_that_reports_no_count() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;
    let create = record(json!({
        "domain_name": "example.com",
        "ipv4_address": "93.184.216.34",
        "source_id": SOURCE_A,
        "first_seen": 1_700_000_000u64
    }))
    .to_request();
    let _: ServiceResponse = test::call_service(&app, create).await;

    let req = test::TestRequest::get()
        .uri("/dns-resolutions/2da44331-2011-5539-b65e-9ff44db9c139")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(body["first_seen"], 1_700_000_000u64);
    assert_eq!(body["last_seen"], 1_700_000_000u64);
    assert_eq!(body["observations"][0]["count"], 1u64);
}

#[tokio::test]
async fn it_lists_one_observation_per_source_and_the_window_that_covers_them() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;
    let first = record_ipv4(
        "example.com",
        "93.184.216.34",
        SOURCE_A,
        1_700_000_000,
        1_700_086_400,
    )
    .to_request();
    let _: ServiceResponse = test::call_service(&app, first).await;
    let second = record_ipv4(
        "example.com",
        "93.184.216.34",
        SOURCE_B,
        1_710_000_000,
        1_710_086_400,
    )
    .to_request();
    let _: ServiceResponse = test::call_service(&app, second).await;

    let req = test::TestRequest::get()
        .uri("/dns-resolutions/2da44331-2011-5539-b65e-9ff44db9c139")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(body["first_seen"], 1_700_000_000u64);
    assert_eq!(body["last_seen"], 1_710_086_400u64);
    let mut sources: Vec<(String, u64, u64)> = body["observations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|observation| {
            (
                observation["source_id"].as_str().unwrap().to_string(),
                observation["first_seen"].as_u64().unwrap(),
                observation["last_seen"].as_u64().unwrap(),
            )
        })
        .collect();
    sources.sort();
    assert_eq!(
        sources,
        vec![
            (SOURCE_A.to_string(), 1_700_000_000, 1_700_086_400),
            (SOURCE_B.to_string(), 1_710_000_000, 1_710_086_400),
        ]
    );
}

#[tokio::test]
async fn it_merges_a_report_into_the_window_and_the_count_of_its_source() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;
    let first = record(json!({
        "domain_name": "example.com",
        "ipv4_address": "93.184.216.34",
        "source_id": SOURCE_A,
        "first_seen": 1_700_000_000u64,
        "last_seen": 1_700_086_400u64,
        "count": 42u64
    }))
    .to_request();
    let _: ServiceResponse = test::call_service(&app, first).await;
    let earlier = record(json!({
        "domain_name": "example.com",
        "ipv4_address": "93.184.216.34",
        "source_id": SOURCE_A,
        "first_seen": 1_690_000_000u64,
        "count": 50u64
    }))
    .to_request();
    let _: ServiceResponse = test::call_service(&app, earlier).await;

    let req = test::TestRequest::get()
        .uri("/dns-resolutions/2da44331-2011-5539-b65e-9ff44db9c139")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    let observations = body["observations"].as_array().unwrap();
    assert_eq!(observations.len(), 1);
    assert_eq!(observations[0]["first_seen"], 1_690_000_000u64);
    assert_eq!(observations[0]["last_seen"], 1_700_086_400u64);
    assert_eq!(observations[0]["count"], 50u64);
}

#[tokio::test]
async fn it_returns_the_canonical_ipv6_answer_with_the_id_of_its_module() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;
    let create = record(json!({
        "domain_name": "example.com",
        "ipv6_address": "2606:2800:0220:0001:0248:1893:25C8:1946",
        "source_id": SOURCE_A,
        "first_seen": 1_700_000_000u64
    }))
    .to_request();
    let _: ServiceResponse = test::call_service(&app, create).await;

    let req = test::TestRequest::get()
        .uri(&format!("/dns-resolutions/{EXAMPLE_COM_TO_IPV6_ID}"))
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(body["stix_id"], EXAMPLE_COM_TO_IPV6_STIX_ID);
    assert_eq!(
        body["answer"],
        json!({
            "kind": "ipv6_address",
            "value": "2606:2800:220:1:248:1893:25c8:1946",
            "id": IPV6_ID
        })
    );
}

#[tokio::test]
async fn it_points_at_the_ids_the_observable_modules_register() {
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
        1_700_000_000,
    )
    .to_request();
    let _: ServiceResponse = test::call_service(&app, create).await;
    let domain_name = test::TestRequest::post()
        .uri("/domain-names")
        .set_json(json!({ "value": "example.com" }))
        .to_request();
    let domain_name: serde_json::Value = test::call_and_read_body_json(&app, domain_name).await;
    let ipv4_address = test::TestRequest::post()
        .uri("/ipv4-addresses")
        .set_json(json!({ "value": "93.184.216.34" }))
        .to_request();
    let ipv4_address: serde_json::Value = test::call_and_read_body_json(&app, ipv4_address).await;

    let req = test::TestRequest::get()
        .uri("/dns-resolutions/2da44331-2011-5539-b65e-9ff44db9c139")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(body["subject"]["id"], domain_name["id"]);
    assert_eq!(body["answer"]["id"], ipv4_address["id"]);
}

#[tokio::test]
async fn it_returns_404_for_a_resolution_that_is_not_recorded() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/dns-resolutions/2da44331-2011-5539-b65e-9ff44db9c139")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

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

    let req = test::TestRequest::get()
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

    let req = test::TestRequest::get()
        .uri(&format!("/dns-resolutions/{SOURCE_A}"))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400);
}
