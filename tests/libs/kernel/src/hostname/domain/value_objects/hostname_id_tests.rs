use kernel::hostname::domain::value_objects::hostname_id::HostnameId;
use uuid::Uuid;

use crate::src::hostname::domain::value_objects::mothers::hostname_id_mother::HostnameIdMother;

fn id_of(value: &str) -> HostnameId {
    HostnameIdMother::create(value)
}

#[test]
fn it_matches_the_opencti_fixture() {
    assert_eq!(
        id_of("mail01.corp.example").stix_id(),
        "hostname--38ee45a4-70cb-5ddf-9cd2-abcd7afa92a2"
    );
}

#[test]
fn it_matches_the_computed_vector() {
    assert_eq!(
        id_of("build-agent-7").stix_id(),
        "hostname--46e518b9-291e-5e2e-b34c-18e18439c5d1"
    );
}

#[test]
fn it_derives_the_same_id_for_every_spelling_of_the_hostname() {
    assert_eq!(
        id_of("MAIL01.Corp.Example."),
        id_of(" mail01.corp.example ")
    );
}

#[test]
fn it_derives_different_ids_for_different_hostnames() {
    assert_ne!(id_of("build-agent-7"), id_of("localhost"));
}

#[test]
fn it_derives_a_version_5_uuid() {
    assert_eq!(id_of("build-agent-7").value().get_version_num(), 5);
}

#[test]
fn it_parses_its_own_string_form() {
    let id = id_of("mail01.corp.example");

    assert_eq!(HostnameId::new(&id.to_string()).expect("valid id"), id);
}

#[test]
fn it_trims_surrounding_whitespace_when_parsing() {
    let id = HostnameId::new("  38ee45a4-70cb-5ddf-9cd2-abcd7afa92a2 ").expect("valid id");

    assert_eq!(id, id_of("mail01.corp.example"));
}

#[test]
fn it_rejects_a_version_4_uuid() {
    let error = HostnameId::from_uuid(Uuid::new_v4()).expect_err("v4 ids are not derived");

    assert!(error.message().contains("must be a UUID v5"));
}

#[test]
fn it_rejects_a_version_5_nibble_with_a_non_rfc_4122_variant() {
    for raw in [
        "00000000-0000-5000-0000-000000000000",
        "00000000-0000-5000-c000-000000000000",
    ] {
        let error = HostnameId::new(raw).expect_err("no derivation produces this UUID");

        assert!(error.message().contains("must be a UUID v5"));
    }
}

#[test]
fn it_rejects_a_value_that_is_not_a_uuid() {
    let error = HostnameId::new("not-a-uuid").expect_err("not a UUID");

    assert!(error.message().contains("not a valid UUID"));
}
