use kernel::ipv6_network::domain::value_objects::ipv6_network_id::Ipv6NetworkId;
use uuid::Uuid;

use crate::src::ipv6_network::domain::value_objects::mothers::ipv6_network_id_mother::Ipv6NetworkIdMother;

fn id_of(value: &str) -> Ipv6NetworkId {
    Ipv6NetworkIdMother::create(value)
}

#[test]
fn it_matches_the_opencti_fixture() {
    assert_eq!(
        id_of("2001:db8::/32").stix_id(),
        "ipv6-addr--8da31feb-e0f9-54c3-ab30-bd8ccea361dd"
    );
}

#[test]
fn it_matches_the_computed_vector() {
    assert_eq!(
        id_of("2001:db8:85a3::/48").stix_id(),
        "ipv6-addr--2a50470b-c9dc-5b93-8fa0-d2c92c80e12e"
    );
}

#[test]
fn it_derives_the_same_id_for_every_spelling_of_the_network() {
    assert_eq!(
        id_of("2001:0DB8:0000:0000:0000:0000:0000:0000/32"),
        id_of(" 2001:db8::/32 ")
    );
}

#[test]
fn it_derives_different_ids_for_different_networks() {
    assert_ne!(id_of("2001:db8:85a3::/48"), id_of("fe80::/10"));
}

#[test]
fn it_derives_a_version_5_uuid() {
    assert_eq!(id_of("2001:db8:85a3::/48").value().get_version_num(), 5);
}

#[test]
fn it_parses_its_own_string_form() {
    let id = id_of("2001:db8::/32");

    assert_eq!(Ipv6NetworkId::new(&id.to_string()).expect("valid id"), id);
}

#[test]
fn it_trims_surrounding_whitespace_when_parsing() {
    let id = Ipv6NetworkId::new("  8da31feb-e0f9-54c3-ab30-bd8ccea361dd ").expect("valid id");

    assert_eq!(id, id_of("2001:db8::/32"));
}

#[test]
fn it_rejects_a_version_4_uuid() {
    let error = Ipv6NetworkId::from_uuid(Uuid::new_v4()).expect_err("v4 ids are not derived");

    assert!(error.message().contains("must be a UUID v5"));
}

#[test]
fn it_rejects_a_version_5_nibble_with_a_non_rfc_4122_variant() {
    for raw in [
        "00000000-0000-5000-0000-000000000000",
        "00000000-0000-5000-c000-000000000000",
    ] {
        let error = Ipv6NetworkId::new(raw).expect_err("no derivation produces this UUID");

        assert!(error.message().contains("must be a UUID v5"));
    }
}

#[test]
fn it_rejects_a_value_that_is_not_a_uuid() {
    let error = Ipv6NetworkId::new("not-a-uuid").expect_err("not a UUID");

    assert!(error.message().contains("not a valid UUID"));
}
