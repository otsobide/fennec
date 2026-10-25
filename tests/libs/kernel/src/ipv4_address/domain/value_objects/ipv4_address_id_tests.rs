use kernel::ipv4_address::domain::value_objects::ipv4_address_id::Ipv4AddressId;
use kernel::ipv4_address::domain::value_objects::ipv4_address_value::Ipv4AddressValue;
use uuid::Uuid;

fn id_of(value: &str) -> Ipv4AddressId {
    Ipv4AddressId::from_value(&Ipv4AddressValue::new(value).expect("valid address"))
}

#[test]
fn it_matches_the_opencti_fixture() {
    assert_eq!(
        id_of("89.19.209.154").stix_id(),
        "ipv4-addr--abfb438a-8baf-5f54-b19e-b85ee74b3ed7"
    );
}

#[test]
fn it_matches_the_computed_vector() {
    assert_eq!(
        id_of("198.51.100.3").stix_id(),
        "ipv4-addr--28bb3599-77cd-5a82-a950-b5bc3caf07c4"
    );
}

#[test]
fn it_derives_the_same_id_for_every_spelling_of_the_address() {
    assert_eq!(id_of("89.19.209.154/32"), id_of(" 89.19.209.154 "));
}

#[test]
fn it_derives_different_ids_for_different_addresses() {
    assert_ne!(id_of("198.51.100.3"), id_of("198.51.100.4"));
}

#[test]
fn it_derives_a_version_5_uuid() {
    assert_eq!(id_of("198.51.100.3").value().get_version_num(), 5);
}

#[test]
fn it_parses_its_own_string_form() {
    let id = id_of("89.19.209.154");

    assert_eq!(Ipv4AddressId::new(&id.to_string()).expect("valid id"), id);
}

#[test]
fn it_trims_surrounding_whitespace_when_parsing() {
    let id = Ipv4AddressId::new("  abfb438a-8baf-5f54-b19e-b85ee74b3ed7 ").expect("valid id");

    assert_eq!(id, id_of("89.19.209.154"));
}

#[test]
fn it_rejects_a_version_4_uuid() {
    let error = Ipv4AddressId::from_uuid(Uuid::new_v4()).expect_err("v4 ids are not derived");

    assert!(error.message().contains("must be a UUID v5"));
}

#[test]
fn it_rejects_a_version_5_nibble_with_a_non_rfc_4122_variant() {
    for raw in [
        "00000000-0000-5000-0000-000000000000",
        "00000000-0000-5000-c000-000000000000",
    ] {
        let error = Ipv4AddressId::new(raw).expect_err("no derivation produces this UUID");

        assert!(error.message().contains("must be a UUID v5"));
    }
}

#[test]
fn it_rejects_a_value_that_is_not_a_uuid() {
    let error = Ipv4AddressId::new("not-a-uuid").expect_err("not a UUID");

    assert!(error.message().contains("not a valid UUID"));
}
