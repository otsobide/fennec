use kernel::ipv6_address::domain::value_objects::ipv6_address_id::Ipv6AddressId;
use uuid::Uuid;

use crate::src::ipv6_address::domain::value_objects::mothers::ipv6_address_id_mother::Ipv6AddressIdMother;

fn id_of(value: &str) -> Ipv6AddressId {
    Ipv6AddressIdMother::create(value)
}

#[test]
fn it_matches_the_opencti_fixture() {
    assert_eq!(
        id_of("2001:db8:85a3::8a2e:370:7334").stix_id(),
        "ipv6-addr--f5344921-72d2-5619-945d-8757bca4b7cd"
    );
}

#[test]
fn it_matches_the_computed_vector() {
    assert_eq!(
        id_of("2001:db8::7").stix_id(),
        "ipv6-addr--9ed76cd5-ab3b-55a5-b6ec-b55c3efb0be2"
    );
}

#[test]
fn it_derives_the_same_id_for_every_spelling_of_the_address() {
    assert_eq!(
        id_of("2001:0DB8:85A3:0000:0000:8A2E:0370:7334/128"),
        id_of(" 2001:db8:85a3::8a2e:370:7334 ")
    );
}

#[test]
fn it_derives_different_ids_for_different_addresses() {
    assert_ne!(id_of("2001:db8::7"), id_of("2001:db8::4"));
}

#[test]
fn it_derives_a_version_5_uuid() {
    assert_eq!(id_of("2001:db8::7").value().get_version_num(), 5);
}

#[test]
fn it_parses_its_own_string_form() {
    let id = id_of("2001:db8:85a3::8a2e:370:7334");

    assert_eq!(Ipv6AddressId::new(&id.to_string()).expect("valid id"), id);
}

#[test]
fn it_trims_surrounding_whitespace_when_parsing() {
    let id = Ipv6AddressId::new("  f5344921-72d2-5619-945d-8757bca4b7cd ").expect("valid id");

    assert_eq!(id, id_of("2001:db8:85a3::8a2e:370:7334"));
}

#[test]
fn it_rejects_a_version_4_uuid() {
    let error = Ipv6AddressId::from_uuid(Uuid::new_v4()).expect_err("v4 ids are not derived");

    assert!(error.message().contains("must be a UUID v5"));
}

#[test]
fn it_rejects_a_version_5_nibble_with_a_non_rfc_4122_variant() {
    for raw in [
        "00000000-0000-5000-0000-000000000000",
        "00000000-0000-5000-c000-000000000000",
    ] {
        let error = Ipv6AddressId::new(raw).expect_err("no derivation produces this UUID");

        assert!(error.message().contains("must be a UUID v5"));
    }
}

#[test]
fn it_rejects_a_value_that_is_not_a_uuid() {
    let error = Ipv6AddressId::new("not-a-uuid").expect_err("not a UUID");

    assert!(error.message().contains("not a valid UUID"));
}
