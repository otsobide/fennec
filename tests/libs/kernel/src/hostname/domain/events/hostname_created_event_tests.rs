use kernel::hostname::domain::events::create_hostname_created_event::create_hostname_created_event;
use kernel::hostname::domain::events::hostname_created_event::HostnameCreatedEvent;
use kernel::hostname::domain::value_objects::hostname_id::HostnameId;
use shared_domain_events::domain::domain_event::DomainEvent;
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::src::hostname::domain::entities::mothers::hostname_mother::HostnameMother;

#[test]
fn it_exposes_the_full_state_as_primitives() {
    let hostname = HostnameMother::create("mail01.corp.example");

    let primitives = create_hostname_created_event(&hostname)
        .expect("event must be created")
        .to_primitives();

    assert_eq!(
        primitives.keys().collect::<Vec<_>>(),
        vec!["created_at", "id", "value"]
    );
    assert_eq!(
        primitives["id"],
        PrimitiveValue::Text("38ee45a4-70cb-5ddf-9cd2-abcd7afa92a2".to_string())
    );
    assert_eq!(
        primitives["value"],
        PrimitiveValue::Text("mail01.corp.example".to_string())
    );
    assert_eq!(
        primitives["created_at"],
        PrimitiveValue::Timestamp(hostname.created_at().value())
    );
}

#[test]
fn it_is_named_after_the_kernel_hostname_module() {
    let event =
        create_hostname_created_event(&HostnameMother::random()).expect("event must be created");

    assert_eq!(event.event_name(), "fennec.kernel.hostname.created");
    assert_eq!(event.event_name(), HostnameCreatedEvent::EVENT_NAME);
}

#[test]
fn it_uses_the_hostname_id_as_aggregate_id() {
    let hostname = HostnameMother::random();

    let event = create_hostname_created_event(&hostname).expect("event must be created");

    assert_eq!(event.aggregate_id(), hostname.id().to_string());
}

#[test]
fn it_carries_the_id_of_its_own_value() {
    let hostname = HostnameMother::random();

    let event = create_hostname_created_event(&hostname).expect("event must be created");

    assert_eq!(event.id(), hostname.id());
    assert_eq!(*event.id(), HostnameId::from_value(event.value()));
}
