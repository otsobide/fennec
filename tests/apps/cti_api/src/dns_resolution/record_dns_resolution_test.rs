use actix_web::{dev::ServiceResponse, test, App};
use serde_json::json;

use cti_api::{build_state, configure_routes};

use crate::src::dns_resolution::support::{
    record, record_ipv4, EXAMPLE_COM_TO_IPV4_ID, EXAMPLE_COM_TO_IPV4_STIX_ID,
    EXAMPLE_COM_TO_IPV6_ID, EXAMPLE_COM_TO_IPV6_STIX_ID, SOURCE_A, SOURCE_B,
};

#[tokio::test]
async fn it_returns_201_with_the_location_and_the_derived_id_for_a_new_resolution() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = record_ipv4(
        "example.com",
        "93.184.216.34",
        SOURCE_A,
        1_700_000_000,
        1_700_086_400,
    )
    .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 201);
    assert_eq!(
        resp.headers().get("Location").unwrap(),
        "/dns-resolutions/2da44331-2011-5539-b65e-9ff44db9c139"
    );
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(
        body,
        json!({
            "id": EXAMPLE_COM_TO_IPV4_ID,
            "stix_id": EXAMPLE_COM_TO_IPV4_STIX_ID,
            "relationship_type": "resolves-to",
            "outcome": "created"
        })
    );
}

#[tokio::test]
async fn it_returns_201_for_a_new_resolution_to_an_ipv6_address() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = record(json!({
        "domain_name": "example.com",
        "ipv6_address": "2606:2800:0220:0001:0248:1893:25C8:1946",
        "source_id": SOURCE_A,
        "first_seen": 1_700_000_000u64
    }))
    .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 201);
    assert_eq!(
        resp.headers().get("Location").unwrap(),
        "/dns-resolutions/5a9e984f-8e13-5693-a8c0-7787f94d751e"
    );
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(
        body,
        json!({
            "id": EXAMPLE_COM_TO_IPV6_ID,
            "stix_id": EXAMPLE_COM_TO_IPV6_STIX_ID,
            "relationship_type": "resolves-to",
            "outcome": "created"
        })
    );
}

#[tokio::test]
async fn it_returns_200_observed_when_a_second_source_reports_the_resolution() {
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
    let resp: ServiceResponse = test::call_service(&app, second).await;

    assert_eq!(resp.status(), 200);
    assert!(resp.headers().get("Location").is_none());
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(
        body,
        json!({
            "id": EXAMPLE_COM_TO_IPV4_ID,
            "stix_id": EXAMPLE_COM_TO_IPV4_STIX_ID,
            "relationship_type": "resolves-to",
            "outcome": "observed"
        })
    );
}

#[tokio::test]
async fn it_returns_200_observed_when_the_same_source_widens_its_window() {
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

    let later = record_ipv4(
        "example.com",
        "93.184.216.34",
        SOURCE_A,
        1_700_086_400,
        1_700_172_800,
    )
    .to_request();
    let resp: ServiceResponse = test::call_service(&app, later).await;

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["outcome"], "observed");
}

#[tokio::test]
async fn it_returns_200_unchanged_when_the_same_report_is_sent_twice() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;
    let report = json!({
        "domain_name": "example.com",
        "ipv4_address": "93.184.216.34",
        "source_id": SOURCE_A,
        "first_seen": 1_700_000_000u64,
        "last_seen": 1_700_086_400u64,
        "count": 42u64
    });
    let first = record(report.clone()).to_request();
    let _: ServiceResponse = test::call_service(&app, first).await;

    let again = record(report).to_request();
    let resp: ServiceResponse = test::call_service(&app, again).await;

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(
        body,
        json!({
            "id": EXAMPLE_COM_TO_IPV4_ID,
            "stix_id": EXAMPLE_COM_TO_IPV4_STIX_ID,
            "relationship_type": "resolves-to",
            "outcome": "unchanged"
        })
    );
}

#[tokio::test]
async fn it_returns_200_unchanged_for_a_report_inside_the_window_with_a_lower_count() {
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

    let inside = record(json!({
        "domain_name": "example.com",
        "ipv4_address": "93.184.216.34",
        "source_id": SOURCE_A,
        "first_seen": 1_700_040_000u64,
        "count": 7u64
    }))
    .to_request();
    let resp: ServiceResponse = test::call_service(&app, inside).await;

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["outcome"], "unchanged");
}

#[tokio::test]
async fn it_converges_every_spelling_of_the_endpoints_on_one_resolution() {
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

    let respelled = record_ipv4(
        " Example.COM. ",
        " 93.184.216.34/32 ",
        SOURCE_A,
        1_700_000_000,
        1_700_086_400,
    )
    .to_request();
    let resp: ServiceResponse = test::call_service(&app, respelled).await;

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(
        body,
        json!({
            "id": EXAMPLE_COM_TO_IPV4_ID,
            "stix_id": EXAMPLE_COM_TO_IPV4_STIX_ID,
            "relationship_type": "resolves-to",
            "outcome": "unchanged"
        })
    );
}

#[tokio::test]
async fn it_returns_400_when_both_address_fields_are_given() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = record(json!({
        "domain_name": "example.com",
        "ipv4_address": "93.184.216.34",
        "ipv6_address": "2606:2800:220:1:248:1893:25c8:1946",
        "source_id": SOURCE_A,
        "first_seen": 1_700_000_000u64
    }))
    .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn it_returns_400_when_no_address_field_is_given() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = record(json!({
        "domain_name": "example.com",
        "source_id": SOURCE_A,
        "first_seen": 1_700_000_000u64
    }))
    .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn it_returns_400_for_an_address_in_the_field_of_the_other_kind() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let ipv6_as_ipv4 = record(json!({
        "domain_name": "example.com",
        "ipv4_address": "2606:2800:220:1:248:1893:25c8:1946",
        "source_id": SOURCE_A,
        "first_seen": 1_700_000_000u64
    }))
    .to_request();
    let resp: ServiceResponse = test::call_service(&app, ipv6_as_ipv4).await;
    assert_eq!(resp.status(), 400);

    let ipv4_as_ipv6 = record(json!({
        "domain_name": "example.com",
        "ipv6_address": "93.184.216.34",
        "source_id": SOURCE_A,
        "first_seen": 1_700_000_000u64
    }))
    .to_request();
    let resp: ServiceResponse = test::call_service(&app, ipv4_as_ipv6).await;
    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn it_returns_400_for_an_invalid_domain_name() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    for domain_name in ["localhost", "93.184.216.34", "exa mple.com", ""] {
        let req = record_ipv4(
            domain_name,
            "93.184.216.34",
            SOURCE_A,
            1_700_000_000,
            1_700_000_000,
        )
        .to_request();
        let resp: ServiceResponse = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400, "{domain_name:?}");
    }
}

#[tokio::test]
async fn it_returns_400_for_an_invalid_address() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    for ipv4_address in ["93.184.216.256", "93.184.216.0/24", "093.184.216.34"] {
        let req = record_ipv4(
            "example.com",
            ipv4_address,
            SOURCE_A,
            1_700_000_000,
            1_700_000_000,
        )
        .to_request();
        let resp: ServiceResponse = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400, "{ipv4_address:?}");
    }

    let zoned = record(json!({
        "domain_name": "example.com",
        "ipv6_address": "fe80::1%eth0",
        "source_id": SOURCE_A,
        "first_seen": 1_700_000_000u64
    }))
    .to_request();
    let resp: ServiceResponse = test::call_service(&app, zoned).await;
    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn it_returns_400_for_a_source_id_that_is_not_a_uuid_v4() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    for source_id in ["not-a-uuid", "2da44331-2011-5539-b65e-9ff44db9c139"] {
        let req = record_ipv4(
            "example.com",
            "93.184.216.34",
            source_id,
            1_700_000_000,
            1_700_000_000,
        )
        .to_request();
        let resp: ServiceResponse = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400, "{source_id:?}");
    }
}

#[tokio::test]
async fn it_returns_400_when_last_seen_is_before_first_seen() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = record_ipv4(
        "example.com",
        "93.184.216.34",
        SOURCE_A,
        1_700_000_001,
        1_700_000_000,
    )
    .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn it_accepts_last_seen_equal_to_first_seen() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = record_ipv4(
        "example.com",
        "93.184.216.34",
        SOURCE_A,
        1_700_000_000,
        1_700_000_000,
    )
    .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 201);
}

#[tokio::test]
async fn it_returns_400_for_an_instant_after_year_9999() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let last_second_of_9999 = 253_402_300_799u64;
    let accepted = record_ipv4(
        "example.com",
        "93.184.216.34",
        SOURCE_A,
        last_second_of_9999,
        last_second_of_9999,
    )
    .to_request();
    let resp: ServiceResponse = test::call_service(&app, accepted).await;
    assert_eq!(resp.status(), 201);

    let rejected = record_ipv4(
        "example.net",
        "93.184.216.34",
        SOURCE_A,
        last_second_of_9999 + 1,
        last_second_of_9999 + 1,
    )
    .to_request();
    let resp: ServiceResponse = test::call_service(&app, rejected).await;
    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn it_returns_400_for_a_negative_instant() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = record(json!({
        "domain_name": "example.com",
        "ipv4_address": "93.184.216.34",
        "source_id": SOURCE_A,
        "first_seen": -1
    }))
    .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn it_returns_400_for_a_count_that_is_not_positive() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    for count in [json!(0), json!(-1), json!(1.5)] {
        let req = record(json!({
            "domain_name": "example.com",
            "ipv4_address": "93.184.216.34",
            "source_id": SOURCE_A,
            "first_seen": 1_700_000_000u64,
            "count": count
        }))
        .to_request();
        let resp: ServiceResponse = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400, "{count}");
    }
}

#[tokio::test]
async fn it_accepts_a_count_of_one() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = record(json!({
        "domain_name": "example.com",
        "ipv4_address": "93.184.216.34",
        "source_id": SOURCE_A,
        "first_seen": 1_700_000_000u64,
        "count": 1u64
    }))
    .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 201);
}

#[tokio::test]
async fn it_returns_400_for_a_count_above_what_a_store_can_hold() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = record(json!({
        "domain_name": "example.com",
        "ipv4_address": "93.184.216.34",
        "source_id": SOURCE_A,
        "first_seen": 1_700_000_000u64,
        "count": i64::MAX as u64 + 1
    }))
    .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn it_returns_400_when_the_client_tries_to_choose_the_id() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = record(json!({
        "id": EXAMPLE_COM_TO_IPV4_ID,
        "domain_name": "example.com",
        "ipv4_address": "93.184.216.34",
        "source_id": SOURCE_A,
        "first_seen": 1_700_000_000u64
    }))
    .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn it_returns_400_for_an_untyped_answer_field() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    let req = record(json!({
        "domain_name": "example.com",
        "answer": "93.184.216.34",
        "source_id": SOURCE_A,
        "first_seen": 1_700_000_000u64
    }))
    .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn it_returns_400_when_a_required_field_is_missing() {
    let app = test::init_service(
        App::new()
            .app_data(build_state())
            .configure(configure_routes),
    )
    .await;

    for missing in ["domain_name", "source_id", "first_seen"] {
        let mut body = json!({
            "domain_name": "example.com",
            "ipv4_address": "93.184.216.34",
            "source_id": SOURCE_A,
            "first_seen": 1_700_000_000u64
        });
        body.as_object_mut().unwrap().remove(missing);
        let resp: ServiceResponse = test::call_service(&app, record(body).to_request()).await;
        assert_eq!(resp.status(), 400, "{missing}");
    }
}
