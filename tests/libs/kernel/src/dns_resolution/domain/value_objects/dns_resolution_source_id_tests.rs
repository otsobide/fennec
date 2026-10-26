use kernel::dns_resolution::domain::value_objects::dns_resolution_source_id::DnsResolutionSourceId;
use uuid::Uuid;

use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_source_id_mother::DnsResolutionSourceIdMother;

const VALID_V4: &str = "550e8400-e29b-41d4-a716-446655440000";

#[test]
fn it_accepts_a_valid_uuid_v4() {
    let id = DnsResolutionSourceId::new(VALID_V4).expect("a UUID v4 must be accepted");

    assert_eq!(id.value().to_string(), VALID_V4);
}

#[test]
fn it_ignores_surrounding_whitespace() {
    let id =
        DnsResolutionSourceId::new(&format!("  {VALID_V4} \n")).expect("whitespace is trimmed");

    assert_eq!(id.to_string(), VALID_V4);
}

#[test]
fn it_accepts_uppercase_hexadecimal_digits() {
    let id = DnsResolutionSourceId::new(&VALID_V4.to_uppercase()).expect("valid id");

    assert_eq!(id.to_string(), VALID_V4);
}

#[test]
fn it_rejects_a_value_that_is_not_a_uuid() {
    let error =
        DnsResolutionSourceId::new("not-a-uuid").expect_err("a malformed value must be rejected");

    assert_eq!(
        error.message(),
        "dns resolution source id is not a valid UUID: not-a-uuid"
    );
}

#[test]
fn it_rejects_an_empty_value() {
    let error = DnsResolutionSourceId::new("  ").expect_err("an empty value must be rejected");

    assert!(error.message().contains("not a valid UUID"));
}

#[test]
fn it_rejects_a_uuid_v1() {
    let error = DnsResolutionSourceId::new("00000000-0000-1000-8000-000000000000")
        .expect_err("a v1 UUID must be rejected");

    assert_eq!(
        error.message(),
        "dns resolution source id must be a UUID v4, got version 1"
    );
}

#[test]
fn it_rejects_a_uuid_v5() {
    let error = DnsResolutionSourceId::new("2da44331-2011-5539-b65e-9ff44db9c139")
        .expect_err("a derived id is not a source id");

    assert_eq!(
        error.message(),
        "dns resolution source id must be a UUID v4, got version 5"
    );
}

#[test]
fn it_rejects_the_nil_uuid() {
    let error = DnsResolutionSourceId::from_uuid(Uuid::nil()).expect_err("nil is not a v4");

    assert!(error.message().contains("got version 0"));
}

#[test]
fn it_wraps_an_already_parsed_uuid_v4() {
    let uuid = Uuid::new_v4();

    let id = DnsResolutionSourceId::from_uuid(uuid).expect("a UUID v4 must be accepted");

    assert_eq!(*id.value(), uuid);
}

#[test]
fn it_generates_a_version_4_identifier() {
    let id = DnsResolutionSourceId::generate();

    assert_eq!(id.value().get_version_num(), 4);
    assert_ne!(id, DnsResolutionSourceId::generate());
}

#[test]
fn it_compares_equal_for_the_same_uuid() {
    assert_eq!(
        DnsResolutionSourceIdMother::create(VALID_V4),
        DnsResolutionSourceIdMother::create(&VALID_V4.to_uppercase())
    );
    assert_ne!(
        DnsResolutionSourceIdMother::create(VALID_V4),
        DnsResolutionSourceIdMother::random()
    );
}
