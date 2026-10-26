use kernel::dns_resolution::domain::value_objects::dns_resolution_answer::DnsResolutionAnswer;
use kernel::dns_resolution::domain::value_objects::dns_resolution_id::DnsResolutionId;
use kernel::dns_resolution::domain::value_objects::dns_resolution_subject::DnsResolutionSubject;
use uuid::{Uuid, Variant};

use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_answer_mother::DnsResolutionAnswerMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_subject_mother::DnsResolutionSubjectMother;

const EXAMPLE_ID: &str = "2da44331-2011-5539-b65e-9ff44db9c139";

fn id_of(subject: &DnsResolutionSubject, answer: &DnsResolutionAnswer) -> DnsResolutionId {
    DnsResolutionId::from_endpoints(subject, answer)
}

fn example() -> DnsResolutionId {
    id_of(
        &DnsResolutionSubjectMother::create("example.com"),
        &DnsResolutionAnswerMother::ipv4("93.184.216.34"),
    )
}

#[test]
fn it_matches_the_documented_vector() {
    assert_eq!(example().to_string(), EXAMPLE_ID);
}

#[test]
fn it_is_named_after_the_dns_resolution_module() {
    assert_eq!(DnsResolutionId::KIND, "dns_resolution");
}

#[test]
fn it_derives_an_rfc_4122_version_5_uuid() {
    let id = id_of(
        &DnsResolutionSubjectMother::random(),
        &DnsResolutionAnswerMother::random(),
    );

    assert_eq!(id.value().get_version_num(), 5);
    assert_eq!(id.value().get_variant(), Variant::RFC4122);
}

#[test]
fn it_derives_the_same_id_for_every_spelling_of_the_endpoints() {
    assert_eq!(
        id_of(
            &DnsResolutionSubjectMother::create(" Example.COM. "),
            &DnsResolutionAnswerMother::ipv4("93.184.216.34/32"),
        ),
        example()
    );
}

#[test]
fn it_derives_different_ids_for_ipv4_and_ipv6_answers() {
    let subject = DnsResolutionSubjectMother::create("example.com");

    assert_ne!(
        id_of(&subject, &DnsResolutionAnswerMother::ipv4("93.184.216.34")),
        id_of(
            &subject,
            &DnsResolutionAnswerMother::ipv6("::ffff:93.184.216.34")
        )
    );
}

#[test]
fn it_derives_different_ids_for_different_answers() {
    let subject = DnsResolutionSubjectMother::create("example.com");

    assert_ne!(
        id_of(&subject, &DnsResolutionAnswerMother::ipv4("93.184.216.34")),
        id_of(&subject, &DnsResolutionAnswerMother::ipv4("93.184.216.35"))
    );
    assert_ne!(
        id_of(&subject, &DnsResolutionAnswerMother::ipv6("2001:db8::7")),
        id_of(&subject, &DnsResolutionAnswerMother::ipv6("2001:db8::8"))
    );
}

#[test]
fn it_derives_different_ids_for_different_subjects() {
    let answer = DnsResolutionAnswerMother::ipv4("93.184.216.34");

    assert_ne!(
        id_of(&DnsResolutionSubjectMother::create("example.com"), &answer),
        id_of(&DnsResolutionSubjectMother::create("example.org"), &answer)
    );
}

#[test]
fn it_differs_from_the_ids_of_its_endpoints() {
    let subject = DnsResolutionSubjectMother::create("example.com");
    let answer = DnsResolutionAnswerMother::ipv4("93.184.216.34");

    let id = id_of(&subject, &answer);

    assert_ne!(*id.value(), subject.id());
    assert_ne!(*id.value(), answer.id());
}

#[test]
fn it_parses_its_own_string_form() {
    let id = example();

    assert_eq!(DnsResolutionId::new(&id.to_string()).expect("valid id"), id);
}

#[test]
fn it_parses_the_documented_vector() {
    assert_eq!(
        DnsResolutionId::new(EXAMPLE_ID).expect("valid id"),
        example()
    );
}

#[test]
fn it_trims_surrounding_whitespace_when_parsing() {
    assert_eq!(
        DnsResolutionId::new(&format!("  {EXAMPLE_ID} \n")).expect("valid id"),
        example()
    );
}

#[test]
fn it_parses_uppercase_hexadecimal_digits() {
    assert_eq!(
        DnsResolutionId::new(&EXAMPLE_ID.to_uppercase()).expect("valid id"),
        example()
    );
}

#[test]
fn it_rejects_a_version_4_uuid() {
    let error =
        DnsResolutionId::new(&Uuid::new_v4().to_string()).expect_err("v4 ids are not derived");

    assert_eq!(
        error.message(),
        "dns resolution id must be a UUID v5 with the RFC 4122 variant, got version 4 with the RFC4122 variant"
    );
}

#[test]
fn it_rejects_the_nil_uuid() {
    let error = DnsResolutionId::new(&Uuid::nil().to_string()).expect_err("nil is not derived");

    assert!(error.message().contains("must be a UUID v5"));
}

#[test]
fn it_rejects_a_version_5_nibble_with_a_non_rfc_4122_variant() {
    for raw in [
        "00000000-0000-5000-0000-000000000000",
        "00000000-0000-5000-c000-000000000000",
        "00000000-0000-5000-e000-000000000000",
    ] {
        let error = DnsResolutionId::new(raw).expect_err("no derivation produces this UUID");

        assert!(
            error.message().contains("must be a UUID v5"),
            "{raw} gave: {}",
            error.message()
        );
    }
}

#[test]
fn it_rejects_a_value_that_is_not_a_uuid() {
    let error = DnsResolutionId::new("not-a-uuid").expect_err("not a UUID");

    assert_eq!(
        error.message(),
        "dns resolution id is not a valid UUID: not-a-uuid"
    );
}

#[test]
fn it_is_exported_as_a_stix_resolves_to_relationship() {
    assert_eq!(DnsResolutionId::STIX_TYPE, "relationship");
    assert_eq!(DnsResolutionId::RELATIONSHIP_TYPE, "resolves-to");
}

#[test]
fn it_builds_the_stix_id_of_the_documented_vector() {
    assert_eq!(
        example().stix_id(),
        "relationship--2da44331-2011-5539-b65e-9ff44db9c139"
    );
}

#[test]
fn it_builds_the_stix_id_from_the_type_and_the_uuid() {
    let id = id_of(
        &DnsResolutionSubjectMother::random(),
        &DnsResolutionAnswerMother::random(),
    );

    assert_eq!(id.stix_id(), format!("relationship--{}", id.value()));
}

#[test]
fn it_displays_the_hyphenated_uuid() {
    let id = example();

    assert_eq!(id.to_string(), id.value().hyphenated().to_string());
}
