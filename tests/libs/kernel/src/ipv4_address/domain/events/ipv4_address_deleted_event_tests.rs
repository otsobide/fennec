use kernel::ipv4_address::domain::events::create_ipv4_address_deleted_event::create_ipv4_address_deleted_event;
use shared_domain_events::domain::domain_event::DomainEvent;
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::src::ipv4_address::domain::entities::mothers::ipv4_address_mother::Ipv4AddressMother;

#[test]
fn it_exposes_the_identity_and_the_value_as_primitives() {
    let ipv4_address = Ipv4AddressMother::create("89.19.209.154");

    let event = create_ipv4_address_deleted_event(&ipv4_address).expect("event must be created");
    let primitives = event.to_primitives();

    assert_eq!(event.event_name(), "fennec.kernel.ipv4_address.deleted");
    assert_eq!(primitives.keys().collect::<Vec<_>>(), vec!["id", "value"]);
    assert_eq!(
        primitives["id"],
        PrimitiveValue::Text("abfb438a-8baf-5f54-b19e-b85ee74b3ed7".to_string())
    );
    assert_eq!(
        primitives["value"],
        PrimitiveValue::Text("89.19.209.154".to_string())
    );
}
