use std::net::Ipv4Addr;

use actix_web::{dev::ServiceResponse, test, App};
use serde_json::json;

use cti_api::{build_state, configure_routes};
use kernel::dns_resolution::application::list_dns_resolutions_by_domain_name::dns_resolutions_by_domain_name_lister::MAX_LISTED;

use crate::src::dns_resolution::support::{
    record, record_ipv4, EXAMPLE_COM_TO_IPV4_ID, EXAMPLE_COM_TO_IPV4_STIX_ID,
    EXAMPLE_COM_TO_IPV6_ID, SOURCE_A, SOURCE_B,
};

/// The answers of the listed resolutions, in the order the API returns them.
fn answers(body: &serde_json::Value) -> Vec<String> {
    body["dns_resolutions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|resolution| resolution["answer"]["value"].as_str().unwrap().to_string())
        .collect()
}

/// The subjects of the listed resolutions, in the order the API returns them.
fn subjects(body: &serde_json::Value) -> Vec<String> {
    body["dns_resolutions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|resolution| resolution["subject"]["value"].as_str().unwrap().to_string())
        .collect()
}

#[tokio::test]
async fn it_lists_the_resolutions_of_a_domain_name_ordered_by_answer() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;
    for address in ["198.51.100.7", "93.184.216.34"] {
        let req = record_ipv4(
            "example.com",
            address,
            SOURCE_A,
            1_700_000_000,
            1_700_000_000,
        )
        .to_request();
        let _: ServiceResponse = test::call_service(&app, req).await;
    }
    let ipv6 = record(json!({
        "domain_name": "example.com",
        "ipv6_address": "2606:2800:220:1:248:1893:25c8:1946",
        "source_id": SOURCE_B,
        "first_seen": 1_700_000_000u64
    }))
    .to_request();
    let _: ServiceResponse = test::call_service(&app, ipv6).await;
    let other = record_ipv4(
        "example.net",
        "93.184.216.34",
        SOURCE_A,
        1_700_000_000,
        1_700_000_000,
    )
    .to_request();
    let _: ServiceResponse = test::call_service(&app, other).await;

    let req = test::TestRequest::get()
        .uri("/dns-resolutions?domain_name=Example.COM")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(
        answers(&body),
        vec![
            "93.184.216.34",
            "198.51.100.7",
            "2606:2800:220:1:248:1893:25c8:1946"
        ]
    );
    assert_eq!(body["truncated"], false);
    let first = &body["dns_resolutions"][0];
    assert_eq!(first["id"], EXAMPLE_COM_TO_IPV4_ID);
    assert_eq!(first["stix_id"], EXAMPLE_COM_TO_IPV4_STIX_ID);
    assert_eq!(first["relationship_type"], "resolves-to");
    assert_eq!(first["subject"]["kind"], "domain_name");
    assert_eq!(first["answer"]["kind"], "ipv4_address");
    assert_eq!(first["observations"][0]["source_id"], SOURCE_A);
    assert_eq!(body["dns_resolutions"][2]["id"], EXAMPLE_COM_TO_IPV6_ID);
    assert_eq!(body["dns_resolutions"][2]["answer"]["kind"], "ipv6_address");
}

#[tokio::test]
async fn it_lists_the_resolutions_to_an_ipv4_address_ordered_by_domain_name() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;
    for domain_name in ["example.org", "example.com", "example.net"] {
        let req = record_ipv4(
            domain_name,
            "93.184.216.34",
            SOURCE_A,
            1_700_000_000,
            1_700_000_000,
        )
        .to_request();
        let _: ServiceResponse = test::call_service(&app, req).await;
    }
    let other = record_ipv4(
        "example.com",
        "198.51.100.7",
        SOURCE_A,
        1_700_000_000,
        1_700_000_000,
    )
    .to_request();
    let _: ServiceResponse = test::call_service(&app, other).await;

    let req = test::TestRequest::get()
        .uri("/dns-resolutions?ipv4_address=93.184.216.34")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(
        subjects(&body),
        vec!["example.com", "example.net", "example.org"]
    );
    assert!(body["dns_resolutions"]
        .as_array()
        .unwrap()
        .iter()
        .all(|resolution| resolution["answer"]["value"] == "93.184.216.34"));
}

#[tokio::test]
async fn it_lists_the_resolutions_to_an_ipv6_address_from_any_spelling() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;
    let create = record(json!({
        "domain_name": "example.com",
        "ipv6_address": "2606:2800:220:1:248:1893:25c8:1946",
        "source_id": SOURCE_A,
        "first_seen": 1_700_000_000u64
    }))
    .to_request();
    let _: ServiceResponse = test::call_service(&app, create).await;

    let req = test::TestRequest::get()
        .uri("/dns-resolutions?ipv6_address=2606:2800:0220:0001:0248:1893:25C8:1946")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(subjects(&body), vec!["example.com"]);
    assert_eq!(body["dns_resolutions"][0]["id"], EXAMPLE_COM_TO_IPV6_ID);
}

#[tokio::test]
async fn it_does_not_list_an_ipv4_mapped_address_as_its_ipv4_address() {
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

    let req = test::TestRequest::get()
        .uri("/dns-resolutions?ipv6_address=::ffff:93.184.216.34")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body, json!({ "dns_resolutions": [], "truncated": false }));
}

#[tokio::test]
async fn it_flags_the_list_as_truncated_only_when_more_than_the_limit_is_recorded() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;
    let address = |index: usize| Ipv4Addr::from(0x0a00_0000 + index as u32).to_string();
    let list = || {
        test::TestRequest::get()
            .uri("/dns-resolutions?domain_name=example.com")
            .to_request()
    };
    for index in 0..MAX_LISTED {
        let req = record_ipv4(
            "example.com",
            &address(index),
            SOURCE_A,
            1_700_000_000,
            1_700_000_000,
        )
        .to_request();
        let resp: ServiceResponse = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 201);
    }

    let at_limit: serde_json::Value = test::call_and_read_body_json(&app, list()).await;

    assert_eq!(answers(&at_limit).len(), MAX_LISTED);
    assert_eq!(at_limit["truncated"], false);

    let one_more = record_ipv4(
        "example.com",
        &address(MAX_LISTED),
        SOURCE_A,
        1_700_000_000,
        1_700_000_000,
    )
    .to_request();
    let resp: ServiceResponse = test::call_service(&app, one_more).await;
    assert_eq!(resp.status(), 201);

    let over_limit: serde_json::Value = test::call_and_read_body_json(&app, list()).await;

    let listed = answers(&over_limit);
    assert_eq!(listed.len(), MAX_LISTED);
    assert_eq!(listed.last(), Some(&address(MAX_LISTED - 1)));
    assert_eq!(over_limit["truncated"], true);
}

#[tokio::test]
async fn it_returns_an_empty_list_when_nothing_matches() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    for uri in [
        "/dns-resolutions?domain_name=example.com",
        "/dns-resolutions?ipv4_address=93.184.216.34",
        "/dns-resolutions?ipv6_address=2001:db8::1",
    ] {
        let req = test::TestRequest::get().uri(uri).to_request();
        let resp: ServiceResponse = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200, "{uri}");
        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(
            body,
            json!({ "dns_resolutions": [], "truncated": false }),
            "{uri}"
        );
    }
}

#[tokio::test]
async fn it_returns_400_without_exactly_one_filter() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    for uri in [
        "/dns-resolutions",
        "/dns-resolutions?domain_name=example.com&ipv4_address=93.184.216.34",
        "/dns-resolutions?ipv4_address=93.184.216.34&ipv6_address=2001:db8::1",
        "/dns-resolutions?domain_name=example.com&ipv4_address=93.184.216.34&ipv6_address=2001:db8::1",
    ] {
        let req = test::TestRequest::get().uri(uri).to_request();
        let resp: ServiceResponse = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400, "{uri}");
    }
}

#[tokio::test]
async fn it_returns_400_for_an_unknown_filter() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    for uri in [
        "/dns-resolutions?value=example.com",
        "/dns-resolutions?domain_name=example.com&source_id=aaaaaaaa-1111-4111-8111-111111111111",
    ] {
        let req = test::TestRequest::get().uri(uri).to_request();
        let resp: ServiceResponse = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400, "{uri}");
    }
}

#[tokio::test]
async fn it_returns_400_for_a_value_that_is_not_valid_for_its_kind() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    for uri in [
        "/dns-resolutions?domain_name=localhost",
        "/dns-resolutions?domain_name=93.184.216.34",
        "/dns-resolutions?ipv4_address=2001:db8::1",
        "/dns-resolutions?ipv4_address=93.184.216.0/24",
        "/dns-resolutions?ipv6_address=93.184.216.34",
        "/dns-resolutions?ipv6_address=example.com",
    ] {
        let req = test::TestRequest::get().uri(uri).to_request();
        let resp: ServiceResponse = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400, "{uri}");
    }
}
