use kernel::dns_resolution::domain::events::create_dns_resolution_retracted_event::create_dns_resolution_retracted_event;
use kernel::dns_resolution::domain::events::dns_resolution_retracted_event::DnsResolutionRetractedEvent;
use shared_domain_events::domain::domain_event::DomainEvent;
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::src::dns_resolution::domain::entities::mothers::dns_resolution_mother::DnsResolutionMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_source_id_mother::DnsResolutionSourceIdMother;

const SOURCE: &str = "550e8400-e29b-41d4-a716-446655440000";

#[test]
fn it_exposes_the_identity_and_the_source_as_primitives() {
    let source_id = DnsResolutionSourceIdMother::create(SOURCE);

    let primitives =
        create_dns_resolution_retracted_event(&DnsResolutionMother::example(), &source_id)
            .expect("event must be created")
            .to_primitives();

    assert_eq!(
        primitives.keys().collect::<Vec<_>>(),
        vec!["id", "source_id"]
    );
    assert_eq!(
        primitives["id"],
        PrimitiveValue::Text("2da44331-2011-5539-b65e-9ff44db9c139".to_string())
    );
    assert_eq!(
        primitives["source_id"],
        PrimitiveValue::Text(SOURCE.to_string())
    );
}

#[test]
fn it_is_named_after_the_kernel_dns_resolution_module() {
    let event = create_dns_resolution_retracted_event(
        &DnsResolutionMother::random(),
        &DnsResolutionSourceIdMother::random(),
    )
    .expect("event must be created");

    assert_eq!(event.event_name(), "fennec.kernel.dns_resolution.retracted");
    assert_eq!(event.event_name(), DnsResolutionRetractedEvent::EVENT_NAME);
}

#[test]
fn it_uses_the_dns_resolution_id_as_aggregate_id() {
    let resolution = DnsResolutionMother::example();
    let source_id = DnsResolutionSourceIdMother::random();

    let event = create_dns_resolution_retracted_event(&resolution, &source_id)
        .expect("event must be created");

    assert_eq!(event.aggregate_id(), "2da44331-2011-5539-b65e-9ff44db9c139");
    assert_eq!(event.id(), resolution.id());
    assert_eq!(*event.source_id(), source_id);
}
