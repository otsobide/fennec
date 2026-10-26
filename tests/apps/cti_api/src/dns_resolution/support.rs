//! Fixtures shared by the DNS resolution e2e tests.

use actix_web::test::TestRequest;
use serde_json::{json, Value};

/// The id of the resolution of `example.com` to `93.184.216.34`
/// (`docs/data-model/links.md`).
pub const EXAMPLE_COM_TO_IPV4_ID: &str = "2da44331-2011-5539-b65e-9ff44db9c139";

/// The STIX 2.1 id of the resolution of `example.com` to `93.184.216.34`,
/// exported as a `relationship`.
pub const EXAMPLE_COM_TO_IPV4_STIX_ID: &str = "relationship--2da44331-2011-5539-b65e-9ff44db9c139";

/// The id of the resolution of `example.com` to
/// `2606:2800:220:1:248:1893:25c8:1946`.
pub const EXAMPLE_COM_TO_IPV6_ID: &str = "5a9e984f-8e13-5693-a8c0-7787f94d751e";

/// The STIX 2.1 id of the resolution of `example.com` to
/// `2606:2800:220:1:248:1893:25c8:1946`, exported as a `relationship`.
pub const EXAMPLE_COM_TO_IPV6_STIX_ID: &str = "relationship--5a9e984f-8e13-5693-a8c0-7787f94d751e";

/// The id `example.com` has in the `domain_name` module
/// (`docs/data-model/kinds/domain-name.md`).
pub const EXAMPLE_COM_ID: &str = "bedb4899-d24b-5401-bc86-8f6b4cc18ec7";

/// The id `93.184.216.34` has in the `ipv4_address` module.
pub const IPV4_ID: &str = "e9778aa6-5cd6-5a44-a746-7d375d1db1e0";

/// The id `2606:2800:220:1:248:1893:25c8:1946` has in the `ipv6_address`
/// module.
pub const IPV6_ID: &str = "83656fe4-138a-5606-a0bb-28a10bc316a1";

/// A source that reports resolutions.
pub const SOURCE_A: &str = "aaaaaaaa-1111-4111-8111-111111111111";

/// Another source that reports resolutions.
pub const SOURCE_B: &str = "bbbbbbbb-2222-4222-8222-222222222222";

/// A `POST /dns-resolutions` request with `body`.
pub fn record(body: Value) -> TestRequest {
    TestRequest::post().uri("/dns-resolutions").set_json(body)
}

/// A `POST /dns-resolutions` request in which `source_id` reports that
/// `domain_name` resolved to the IPv4 address `ipv4_address` from
/// `first_seen` to `last_seen`.
pub fn record_ipv4(
    domain_name: &str,
    ipv4_address: &str,
    source_id: &str,
    first_seen: u64,
    last_seen: u64,
) -> TestRequest {
    record(json!({
        "domain_name": domain_name,
        "ipv4_address": ipv4_address,
        "source_id": source_id,
        "first_seen": first_seen,
        "last_seen": last_seen
    }))
}
