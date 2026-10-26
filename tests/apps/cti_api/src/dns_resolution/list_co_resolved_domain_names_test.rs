use actix_web::{dev::ServiceResponse, test, App};
use serde_json::json;

use cti_api::{build_state, configure_routes};

use crate::src::dns_resolution::support::{
    record, record_ipv4, EXAMPLE_COM_ID, IPV4_ID, IPV6_ID, SOURCE_A, SOURCE_B,
};

#[tokio::test]
async fn it_lists_the_names_that_shared_an_address_at_overlapping_times() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;
    for (domain_name, address, source_id, first_seen, last_seen) in [
        // The queried name, on two addresses.
        (
            "example.com",
            "93.184.216.34",
            SOURCE_A,
            1_700_000_000,
            1_700_086_400,
        ),
        (
            "example.com",
            "198.51.100.7",
            SOURCE_A,
            1_700_000_000,
            1_700_086_400,
        ),
        // Overlapping on both addresses.
        (
            "example.net",
            "198.51.100.7",
            SOURCE_B,
            1_700_040_000,
            1_700_100_000,
        ),
        (
            "example.net",
            "93.184.216.34",
            SOURCE_B,
            1_699_000_000,
            1_700_000_000,
        ),
        // Overlapping on one address only.
        (
            "www.example.com",
            "198.51.100.7",
            SOURCE_A,
            1_700_050_000,
            1_700_050_000,
        ),
        // Years later on the same address: not co-resolved.
        (
            "example.org",
            "93.184.216.34",
            SOURCE_B,
            1_800_000_000,
            1_800_086_400,
        ),
        // Another address altogether.
        (
            "example.edu",
            "203.0.113.9",
            SOURCE_A,
            1_700_000_000,
            1_700_086_400,
        ),
    ] {
        let req = record_ipv4(domain_name, address, source_id, first_seen, last_seen).to_request();
        let resp: ServiceResponse = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 201, "{domain_name} {address}");
    }

    let req = test::TestRequest::get()
        .uri("/dns-resolutions/co-resolved?domain_name=Example.COM")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(
        body,
        json!({
            "domain_name": { "kind": "domain_name", "value": "example.com", "id": EXAMPLE_COM_ID },
            "co_resolved": [
                {
                    "domain_name": {
                        "kind": "domain_name",
                        "value": "example.net",
                        "id": "8d445db5-71d7-5c41-be24-5649e641fb61"
                    },
                    "shared_answers": [
                        { "kind": "ipv4_address", "value": "93.184.216.34", "id": IPV4_ID },
                        {
                            "kind": "ipv4_address",
                            "value": "198.51.100.7",
                            "id": "12592121-be4c-5a93-a813-5a5af16e50db"
                        }
                    ]
                },
                {
                    "domain_name": {
                        "kind": "domain_name",
                        "value": "www.example.com",
                        "id": "3b96539c-807f-59c9-b4df-fe1865757b1e"
                    },
                    "shared_answers": [
                        {
                            "kind": "ipv4_address",
                            "value": "198.51.100.7",
                            "id": "12592121-be4c-5a93-a813-5a5af16e50db"
                        }
                    ]
                }
            ],
            "truncated": false
        })
    );
}

#[tokio::test]
async fn it_treats_windows_that_touch_at_one_instant_as_overlapping() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;
    for (domain_name, first_seen, last_seen) in [
        ("example.com", 1_700_000_000, 1_700_086_400),
        ("example.net", 1_700_086_400, 1_700_200_000),
        ("example.org", 1_700_086_401, 1_700_200_000),
        ("example.edu", 1_699_000_000, 1_699_999_999),
    ] {
        let req = record_ipv4(
            domain_name,
            "93.184.216.34",
            SOURCE_A,
            first_seen,
            last_seen,
        )
        .to_request();
        let _: ServiceResponse = test::call_service(&app, req).await;
    }

    let req = test::TestRequest::get()
        .uri("/dns-resolutions/co-resolved?domain_name=example.com")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    let names: Vec<&str> = body["co_resolved"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| entry["domain_name"]["value"].as_str().unwrap())
        .collect();
    assert_eq!(names, vec!["example.net"]);
}

#[tokio::test]
async fn it_compares_the_windows_that_cover_every_source() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;
    // Source A saw example.com long before example.net resolved there;
    // source B saw example.com again at the same time.
    for (domain_name, source_id, first_seen, last_seen) in [
        ("example.com", SOURCE_A, 1_600_000_000, 1_600_086_400),
        ("example.net", SOURCE_A, 1_700_000_000, 1_700_086_400),
    ] {
        let req = record_ipv4(
            domain_name,
            "93.184.216.34",
            source_id,
            first_seen,
            last_seen,
        )
        .to_request();
        let _: ServiceResponse = test::call_service(&app, req).await;
    }
    let before = test::TestRequest::get()
        .uri("/dns-resolutions/co-resolved?domain_name=example.com")
        .to_request();
    let before: serde_json::Value = test::call_and_read_body_json(&app, before).await;
    assert_eq!(before["co_resolved"], json!([]));

    let later = record_ipv4(
        "example.com",
        "93.184.216.34",
        SOURCE_B,
        1_700_050_000,
        1_700_050_000,
    )
    .to_request();
    let _: ServiceResponse = test::call_service(&app, later).await;

    let after = test::TestRequest::get()
        .uri("/dns-resolutions/co-resolved?domain_name=example.com")
        .to_request();
    let after: serde_json::Value = test::call_and_read_body_json(&app, after).await;
    assert_eq!(
        after["co_resolved"][0]["domain_name"]["value"],
        "example.net"
    );
}

#[tokio::test]
async fn it_pivots_through_ipv6_answers() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;
    for domain_name in ["example.com", "example.net"] {
        let req = record(json!({
            "domain_name": domain_name,
            "ipv6_address": "2606:2800:220:1:248:1893:25c8:1946",
            "source_id": SOURCE_A,
            "first_seen": 1_700_000_000u64
        }))
        .to_request();
        let _: ServiceResponse = test::call_service(&app, req).await;
    }

    let req = test::TestRequest::get()
        .uri("/dns-resolutions/co-resolved?domain_name=example.net")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(
        body["co_resolved"],
        json!([
            {
                "domain_name": { "kind": "domain_name", "value": "example.com", "id": EXAMPLE_COM_ID },
                "shared_answers": [
                    {
                        "kind": "ipv6_address",
                        "value": "2606:2800:220:1:248:1893:25c8:1946",
                        "id": IPV6_ID
                    }
                ]
            }
        ])
    );
}

#[tokio::test]
async fn it_returns_200_with_no_co_resolved_names_for_a_name_without_resolutions() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/dns-resolutions/co-resolved?domain_name=example.com")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(
        body,
        json!({
            "domain_name": { "kind": "domain_name", "value": "example.com", "id": EXAMPLE_COM_ID },
            "co_resolved": [],
            "truncated": false
        })
    );
}

#[tokio::test]
async fn it_returns_400_for_an_invalid_domain_name() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    for uri in [
        "/dns-resolutions/co-resolved?domain_name=localhost",
        "/dns-resolutions/co-resolved?domain_name=93.184.216.34",
    ] {
        let req = test::TestRequest::get().uri(uri).to_request();
        let resp: ServiceResponse = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400, "{uri}");
    }
}

#[tokio::test]
async fn it_returns_400_without_a_domain_name_or_with_an_unknown_parameter() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    for uri in [
        "/dns-resolutions/co-resolved",
        "/dns-resolutions/co-resolved?ipv4_address=93.184.216.34",
        "/dns-resolutions/co-resolved?domain_name=example.com&limit=10",
    ] {
        let req = test::TestRequest::get().uri(uri).to_request();
        let resp: ServiceResponse = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400, "{uri}");
    }
}
