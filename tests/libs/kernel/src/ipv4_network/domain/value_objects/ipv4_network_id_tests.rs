use kernel::ipv4_network::domain::value_objects::ipv4_network_id::Ipv4NetworkId;
use uuid::Uuid;

use crate::src::ipv4_network::domain::value_objects::mothers::ipv4_network_id_mother::Ipv4NetworkIdMother;

fn id_of(value: &str) -> Ipv4NetworkId {
    Ipv4NetworkIdMother::create(value)
}

#[test]
fn it_matches_the_opencti_fixture() {
    assert_eq!(
        id_of("198.51.100.0/24").stix_id(),
        "ipv4-addr--1ea47840-aa39-5aa8-b4a2-cd203eb6662a"
    );
}

#[test]
fn it_matches_the_computed_vector() {
    assert_eq!(
        id_of("203.0.113.0/24").stix_id(),
        "ipv4-addr--a0895f10-d89d-56ad-a4a4-dc873ee08b8c"
    );
}

#[test]
fn it_derives_the_same_id_for_every_spelling_of_the_network() {
    assert_eq!(id_of("198.51.100.0/24  "), id_of(" 198.51.100.0/24 "));
}

#[test]
fn it_derives_different_ids_for_different_networks() {
    assert_ne!(id_of("203.0.113.0/24"), id_of("192.0.2.0/24"));
}

#[test]
fn it_derives_a_version_5_uuid() {
    assert_eq!(id_of("203.0.113.0/24").value().get_version_num(), 5);
}

#[test]
fn it_parses_its_own_string_form() {
    let id = id_of("198.51.100.0/24");

    assert_eq!(Ipv4NetworkId::new(&id.to_string()).expect("valid id"), id);
}

#[test]
fn it_trims_surrounding_whitespace_when_parsing() {
    let id = Ipv4NetworkId::new("  1ea47840-aa39-5aa8-b4a2-cd203eb6662a ").expect("valid id");

    assert_eq!(id, id_of("198.51.100.0/24"));
}

#[test]
fn it_rejects_a_version_4_uuid() {
    let error = Ipv4NetworkId::from_uuid(Uuid::new_v4()).expect_err("v4 ids are not derived");

    assert!(error.message().contains("must be a UUID v5"));
}

#[test]
fn it_rejects_a_version_5_nibble_with_a_non_rfc_4122_variant() {
    for raw in [
        "00000000-0000-5000-0000-000000000000",
        "00000000-0000-5000-c000-000000000000",
    ] {
        let error = Ipv4NetworkId::new(raw).expect_err("no derivation produces this UUID");

        assert!(error.message().contains("must be a UUID v5"));
    }
}

#[test]
fn it_rejects_a_value_that_is_not_a_uuid() {
    let error = Ipv4NetworkId::new("not-a-uuid").expect_err("not a UUID");

    assert!(error.message().contains("not a valid UUID"));
}
