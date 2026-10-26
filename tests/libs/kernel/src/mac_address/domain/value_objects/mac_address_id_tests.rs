use kernel::mac_address::domain::value_objects::mac_address_id::MacAddressId;
use uuid::Uuid;

use crate::src::mac_address::domain::value_objects::mothers::mac_address_id_mother::MacAddressIdMother;

fn id_of(value: &str) -> MacAddressId {
    MacAddressIdMother::create(value)
}

#[test]
fn it_matches_the_opencti_fixture() {
    assert_eq!(
        id_of("d2:fb:49:24:37:18").stix_id(),
        "mac-addr--757b1725-9903-54f5-a855-1240691d7659"
    );
}

#[test]
fn it_matches_the_computed_vector() {
    assert_eq!(
        id_of("00:1b:63:84:45:e6").stix_id(),
        "mac-addr--46e4e100-1877-5fe6-a06f-c77917889a2b"
    );
}

#[test]
fn it_derives_the_same_id_for_every_spelling_of_the_address() {
    assert_eq!(id_of("D2-FB-49-24-37-18"), id_of(" d2:fb:49:24:37:18 "));
}

#[test]
fn it_derives_different_ids_for_different_addresses() {
    assert_ne!(id_of("00:1b:63:84:45:e6"), id_of("ff:ff:ff:ff:ff:ff"));
}

#[test]
fn it_derives_a_version_5_uuid() {
    assert_eq!(id_of("00:1b:63:84:45:e6").value().get_version_num(), 5);
}

#[test]
fn it_parses_its_own_string_form() {
    let id = id_of("d2:fb:49:24:37:18");

    assert_eq!(MacAddressId::new(&id.to_string()).expect("valid id"), id);
}

#[test]
fn it_trims_surrounding_whitespace_when_parsing() {
    let id = MacAddressId::new("  757b1725-9903-54f5-a855-1240691d7659 ").expect("valid id");

    assert_eq!(id, id_of("d2:fb:49:24:37:18"));
}

#[test]
fn it_rejects_a_version_4_uuid() {
    let error = MacAddressId::from_uuid(Uuid::new_v4()).expect_err("v4 ids are not derived");

    assert!(error.message().contains("must be a UUID v5"));
}

#[test]
fn it_rejects_a_version_5_nibble_with_a_non_rfc_4122_variant() {
    for raw in [
        "00000000-0000-5000-0000-000000000000",
        "00000000-0000-5000-c000-000000000000",
    ] {
        let error = MacAddressId::new(raw).expect_err("no derivation produces this UUID");

        assert!(error.message().contains("must be a UUID v5"));
    }
}

#[test]
fn it_rejects_a_value_that_is_not_a_uuid() {
    let error = MacAddressId::new("not-a-uuid").expect_err("not a UUID");

    assert!(error.message().contains("not a valid UUID"));
}
