use shared_valueobject::domain::canonical_json::canonical_json_value::CanonicalJsonValue;
use shared_valueobject::domain::stix::stix_deterministic_uuid::StixDeterministicUuid;

fn uuid_of(properties: CanonicalJsonValue) -> String {
    StixDeterministicUuid::from_contributing_properties(&properties).to_string()
}

fn value(value: &str) -> CanonicalJsonValue {
    CanonicalJsonValue::object([("value", CanonicalJsonValue::string(value))])
}

#[test]
fn it_uses_the_oasis_namespace() {
    assert_eq!(
        StixDeterministicUuid::NAMESPACE.to_string(),
        "00abedb4-aa42-466c-9c01-fed23315a9b7"
    );
}

#[test]
fn it_produces_a_version_5_uuid() {
    let uuid = StixDeterministicUuid::from_contributing_properties(&value("example.com"));

    assert_eq!(uuid.value().get_version_num(), 5);
}

#[test]
fn it_matches_the_opencti_ipv4_address_fixture() {
    assert_eq!(
        uuid_of(value("89.19.209.154")),
        "abfb438a-8baf-5f54-b19e-b85ee74b3ed7"
    );
}

#[test]
fn it_matches_the_opencti_domain_name_fixture() {
    assert_eq!(
        uuid_of(value("hostelhotels.net")),
        "07f3ac51-335f-524c-918f-86f80da2bce3"
    );
}

#[test]
fn it_matches_the_opencti_file_md5_fixture() {
    let properties = CanonicalJsonValue::object([(
        "hashes",
        CanonicalJsonValue::object([(
            "MD5",
            CanonicalJsonValue::string("025ad219ece1125a8f5a0e74e32676cb"),
        )]),
    )]);

    assert_eq!(uuid_of(properties), "cd03138e-eb70-5409-b5df-2f53bee7a1e1");
}

#[test]
fn it_matches_the_opencti_file_sha256_fixture() {
    let properties = CanonicalJsonValue::object([(
        "hashes",
        CanonicalJsonValue::object([(
            "SHA-256",
            CanonicalJsonValue::string(
                "aba8b59281faa8c1c43a4ca7af075edd3e3516d3cef058a1f43b093177b8f83c",
            ),
        )]),
    )]);

    assert_eq!(uuid_of(properties), "1b6bb8bc-bc12-5fe7-9d5c-43fce2a4ac84");
}

#[test]
fn it_matches_the_python_stix2_registry_values_fixture() {
    let properties = CanonicalJsonValue::object([(
        "values",
        CanonicalJsonValue::array([
            CanonicalJsonValue::object([
                ("name", CanonicalJsonValue::string("Foo")),
                ("data", CanonicalJsonValue::string("qwerty")),
            ]),
            CanonicalJsonValue::object([
                ("name", CanonicalJsonValue::string("Bar")),
                ("data", CanonicalJsonValue::string("42")),
            ]),
        ]),
    )]);

    assert_eq!(uuid_of(properties), "36594eba-bcc7-5014-9835-0e154264e588");
}

#[test]
fn it_hashes_integer_contributing_properties() {
    let properties = CanonicalJsonValue::object([(
        "number",
        CanonicalJsonValue::integer(15139).expect("valid integer"),
    )]);

    assert_eq!(uuid_of(properties), "3aa27478-50b5-5ab8-9da9-cdc12b657fff");
}

#[test]
fn it_hashes_non_ascii_values_as_utf8() {
    assert_eq!(
        uuid_of(value("xn--fa-hia.de")),
        "1c80db4c-9024-51b7-a295-c50e9bf33964"
    );
}

#[test]
fn it_does_not_depend_on_member_order() {
    let forward = CanonicalJsonValue::object([
        ("body", CanonicalJsonValue::string("World")),
        ("subject", CanonicalJsonValue::string("Hello")),
    ]);
    let backward = CanonicalJsonValue::object([
        ("subject", CanonicalJsonValue::string("Hello")),
        ("body", CanonicalJsonValue::string("World")),
    ]);

    assert_eq!(uuid_of(forward), uuid_of(backward));
}

#[test]
fn it_gives_different_values_different_uuids() {
    assert_ne!(
        uuid_of(value("198.51.100.3")),
        uuid_of(value("198.51.100.4"))
    );
}
