use kernel::dns_resolution::domain::events::create_dns_resolution_deleted_event::create_dns_resolution_deleted_event;
use kernel::dns_resolution::domain::events::dns_resolution_deleted_event::DnsResolutionDeletedEvent;
use shared_domain_events::domain::domain_event::DomainEvent;
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::src::dns_resolution::domain::entities::mothers::dns_resolution_mother::DnsResolutionMother;

fn text(value: &str) -> PrimitiveValue {
    PrimitiveValue::Text(value.to_string())
}

#[test]
fn it_exposes_the_identity_and_both_endpoints_as_primitives() {
    let primitives = create_dns_resolution_deleted_event(&DnsResolutionMother::example())
        .expect("event must be created")
        .to_primitives();

    assert_eq!(
        primitives.keys().collect::<Vec<_>>(),
        vec![
            "answer_id",
            "answer_kind",
            "answer_value",
            "id",
            "subject_id",
            "subject_kind",
            "subject_value"
        ]
    );
    assert_eq!(
        primitives["id"],
        text("2da44331-2011-5539-b65e-9ff44db9c139")
    );
    assert_eq!(primitives["subject_kind"], text("domain_name"));
    assert_eq!(primitives["subject_value"], text("example.com"));
    assert_eq!(
        primitives["subject_id"],
        text("bedb4899-d24b-5401-bc86-8f6b4cc18ec7")
    );
    assert_eq!(primitives["answer_kind"], text("ipv4_address"));
    assert_eq!(primitives["answer_value"], text("93.184.216.34"));
    assert_eq!(
        primitives["answer_id"],
        text("e9778aa6-5cd6-5a44-a746-7d375d1db1e0")
    );
}

#[test]
fn it_is_named_after_the_kernel_dns_resolution_module() {
    let event = create_dns_resolution_deleted_event(&DnsResolutionMother::random())
        .expect("event must be created");

    assert_eq!(event.event_name(), "fennec.kernel.dns_resolution.deleted");
    assert_eq!(event.event_name(), DnsResolutionDeletedEvent::EVENT_NAME);
}

#[test]
fn it_uses_the_dns_resolution_id_as_aggregate_id() {
    let resolution = DnsResolutionMother::example();

    let event = create_dns_resolution_deleted_event(&resolution).expect("event must be created");

    assert_eq!(event.aggregate_id(), "2da44331-2011-5539-b65e-9ff44db9c139");
    assert_eq!(event.id(), resolution.id());
}
