use kernel::email_address::domain::value_objects::email_address_id::EmailAddressId;
use uuid::Uuid;

use crate::src::email_address::domain::value_objects::mothers::email_address_id_mother::EmailAddressIdMother;

fn id_of(value: &str) -> EmailAddressId {
    EmailAddressIdMother::create(value)
}

#[test]
fn it_matches_the_opencti_fixture() {
    assert_eq!(
        id_of("john@example.com").stix_id(),
        "email-addr--7165e2a9-671f-585d-b1e1-ca59c671d934"
    );
}

#[test]
fn it_matches_the_computed_vector() {
    assert_eq!(
        id_of("billing@xn--bcher-kva.example").stix_id(),
        "email-addr--cf72105f-c27b-5289-bada-a2f65b957880"
    );
}

#[test]
fn it_derives_the_same_id_for_every_spelling_of_the_address() {
    assert_eq!(id_of("John@Example.COM"), id_of(" john@example.com "));
}

#[test]
fn it_derives_different_ids_for_different_addresses() {
    assert_ne!(
        id_of("billing@xn--bcher-kva.example"),
        id_of("noreply@example.net")
    );
}

#[test]
fn it_derives_a_version_5_uuid() {
    assert_eq!(
        id_of("billing@xn--bcher-kva.example")
            .value()
            .get_version_num(),
        5
    );
}

#[test]
fn it_parses_its_own_string_form() {
    let id = id_of("john@example.com");

    assert_eq!(EmailAddressId::new(&id.to_string()).expect("valid id"), id);
}

#[test]
fn it_trims_surrounding_whitespace_when_parsing() {
    let id = EmailAddressId::new("  7165e2a9-671f-585d-b1e1-ca59c671d934 ").expect("valid id");

    assert_eq!(id, id_of("john@example.com"));
}

#[test]
fn it_rejects_a_version_4_uuid() {
    let error = EmailAddressId::from_uuid(Uuid::new_v4()).expect_err("v4 ids are not derived");

    assert!(error.message().contains("must be a UUID v5"));
}

#[test]
fn it_rejects_a_version_5_nibble_with_a_non_rfc_4122_variant() {
    for raw in [
        "00000000-0000-5000-0000-000000000000",
        "00000000-0000-5000-c000-000000000000",
    ] {
        let error = EmailAddressId::new(raw).expect_err("no derivation produces this UUID");

        assert!(error.message().contains("must be a UUID v5"));
    }
}

#[test]
fn it_rejects_a_value_that_is_not_a_uuid() {
    let error = EmailAddressId::new("not-a-uuid").expect_err("not a UUID");

    assert!(error.message().contains("not a valid UUID"));
}
