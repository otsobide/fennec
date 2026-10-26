use kernel::ipv6_network::domain::events::create_ipv6_network_deleted_event::create_ipv6_network_deleted_event;
use kernel::ipv6_network::domain::value_objects::ipv6_network_id::Ipv6NetworkId;
use shared_domain_events::domain::domain_event::DomainEvent;
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::src::ipv6_network::domain::entities::mothers::ipv6_network_mother::Ipv6NetworkMother;

#[test]
fn it_exposes_the_identity_and_the_value_as_primitives() {
    let ipv6_network = Ipv6NetworkMother::create("2001:db8::/32");

    let event = create_ipv6_network_deleted_event(&ipv6_network).expect("event must be created");
    let primitives = event.to_primitives();

    assert_eq!(event.event_name(), "fennec.kernel.ipv6_network.deleted");
    assert_eq!(primitives.keys().collect::<Vec<_>>(), vec!["id", "value"]);
    assert_eq!(
        primitives["id"],
        PrimitiveValue::Text("8da31feb-e0f9-54c3-ab30-bd8ccea361dd".to_string())
    );
    assert_eq!(
        primitives["value"],
        PrimitiveValue::Text("2001:db8::/32".to_string())
    );
}

#[test]
fn it_carries_the_id_of_its_own_value() {
    let ipv6_network = Ipv6NetworkMother::random();

    let event = create_ipv6_network_deleted_event(&ipv6_network).expect("event must be created");

    assert_eq!(event.id(), ipv6_network.id());
    assert_eq!(*event.id(), Ipv6NetworkId::from_value(event.value()));
}
