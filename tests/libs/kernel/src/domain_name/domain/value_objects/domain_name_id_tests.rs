use kernel::domain_name::domain::value_objects::domain_name_id::DomainNameId;
use uuid::Uuid;

use crate::src::domain_name::domain::value_objects::mothers::domain_name_id_mother::DomainNameIdMother;

fn id_of(value: &str) -> DomainNameId {
    DomainNameIdMother::create(value)
}

#[test]
fn it_matches_the_opencti_fixture() {
    assert_eq!(
        id_of("hostelhotels.net").stix_id(),
        "domain-name--07f3ac51-335f-524c-918f-86f80da2bce3"
    );
}

#[test]
fn it_matches_the_computed_vector() {
    assert_eq!(
        id_of("example.com").stix_id(),
        "domain-name--bedb4899-d24b-5401-bc86-8f6b4cc18ec7"
    );
}

#[test]
fn it_derives_the_same_id_for_every_spelling_of_the_name() {
    assert_eq!(id_of("HostelHotels.NET."), id_of(" hostelhotels.net "));
}

#[test]
fn it_derives_different_ids_for_different_names() {
    assert_ne!(id_of("example.com"), id_of("example.org"));
}

#[test]
fn it_derives_a_version_5_uuid() {
    assert_eq!(id_of("example.com").value().get_version_num(), 5);
}

#[test]
fn it_parses_its_own_string_form() {
    let id = id_of("hostelhotels.net");

    assert_eq!(DomainNameId::new(&id.to_string()).expect("valid id"), id);
}

#[test]
fn it_trims_surrounding_whitespace_when_parsing() {
    let id = DomainNameId::new("  07f3ac51-335f-524c-918f-86f80da2bce3 ").expect("valid id");

    assert_eq!(id, id_of("hostelhotels.net"));
}

#[test]
fn it_rejects_a_version_4_uuid() {
    let error = DomainNameId::from_uuid(Uuid::new_v4()).expect_err("v4 ids are not derived");

    assert!(error.message().contains("must be a UUID v5"));
}

#[test]
fn it_rejects_a_version_5_nibble_with_a_non_rfc_4122_variant() {
    for raw in [
        "00000000-0000-5000-0000-000000000000",
        "00000000-0000-5000-c000-000000000000",
    ] {
        let error = DomainNameId::new(raw).expect_err("no derivation produces this UUID");

        assert!(error.message().contains("must be a UUID v5"));
    }
}

#[test]
fn it_rejects_a_value_that_is_not_a_uuid() {
    let error = DomainNameId::new("not-a-uuid").expect_err("not a UUID");

    assert!(error.message().contains("not a valid UUID"));
}
