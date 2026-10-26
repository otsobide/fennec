use shared_valueobject::domain::canonical_json::canonical_json_value::CanonicalJsonValue;
use shared_valueobject::domain::identity::deterministic_uuid::DeterministicUuid;
use shared_valueobject::domain::identity::fennec_link_namespace::FENNEC_LINK_NAMESPACE;
use shared_valueobject::domain::stix::stix_deterministic_uuid::StixDeterministicUuid;
use uuid::Uuid;

fn endpoint(kind: &str, id: &str) -> CanonicalJsonValue {
    CanonicalJsonValue::object([
        ("id", CanonicalJsonValue::string(id)),
        ("kind", CanonicalJsonValue::string(kind)),
    ])
}

#[test]
fn it_derives_the_link_namespace_from_its_urn() {
    assert_eq!(
        FENNEC_LINK_NAMESPACE,
        Uuid::new_v5(&Uuid::NAMESPACE_URL, b"urn:fennec:link")
    );
}

#[test]
fn it_matches_the_documented_link_vector() {
    let name = CanonicalJsonValue::object([
        (
            "answer",
            endpoint("ipv4_address", "e9778aa6-5cd6-5a44-a746-7d375d1db1e0"),
        ),
        ("kind", CanonicalJsonValue::string("dns_resolution")),
        (
            "subject",
            endpoint("domain_name", "bedb4899-d24b-5401-bc86-8f6b4cc18ec7"),
        ),
    ]);

    assert_eq!(
        DeterministicUuid::derive(&FENNEC_LINK_NAMESPACE, &name).to_string(),
        "2da44331-2011-5539-b65e-9ff44db9c139"
    );
}

#[test]
fn it_does_not_depend_on_member_order() {
    let forwards = CanonicalJsonValue::object([
        ("a", CanonicalJsonValue::string("1")),
        ("b", CanonicalJsonValue::string("2")),
    ]);
    let backwards = CanonicalJsonValue::object([
        ("b", CanonicalJsonValue::string("2")),
        ("a", CanonicalJsonValue::string("1")),
    ]);

    assert_eq!(
        DeterministicUuid::derive(&FENNEC_LINK_NAMESPACE, &forwards),
        DeterministicUuid::derive(&FENNEC_LINK_NAMESPACE, &backwards)
    );
}

#[test]
fn it_gives_the_stix_uuid_under_the_oasis_namespace() {
    let properties =
        CanonicalJsonValue::object([("value", CanonicalJsonValue::string("example.com"))]);

    assert_eq!(
        DeterministicUuid::derive(&StixDeterministicUuid::NAMESPACE, &properties).value(),
        StixDeterministicUuid::from_contributing_properties(&properties).value()
    );
}

#[test]
fn it_separates_namespaces() {
    let properties =
        CanonicalJsonValue::object([("value", CanonicalJsonValue::string("example.com"))]);

    assert_ne!(
        DeterministicUuid::derive(&FENNEC_LINK_NAMESPACE, &properties).value(),
        StixDeterministicUuid::from_contributing_properties(&properties).value()
    );
}
