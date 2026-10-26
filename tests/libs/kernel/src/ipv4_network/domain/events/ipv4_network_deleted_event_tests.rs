use kernel::ipv4_network::domain::events::create_ipv4_network_deleted_event::create_ipv4_network_deleted_event;
use kernel::ipv4_network::domain::value_objects::ipv4_network_id::Ipv4NetworkId;
use shared_domain_events::domain::domain_event::DomainEvent;
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::src::ipv4_network::domain::entities::mothers::ipv4_network_mother::Ipv4NetworkMother;

#[test]
fn it_exposes_the_identity_and_the_value_as_primitives() {
    let ipv4_network = Ipv4NetworkMother::create("198.51.100.0/24");

    let event = create_ipv4_network_deleted_event(&ipv4_network).expect("event must be created");
    let primitives = event.to_primitives();

    assert_eq!(event.event_name(), "fennec.kernel.ipv4_network.deleted");
    assert_eq!(primitives.keys().collect::<Vec<_>>(), vec!["id", "value"]);
    assert_eq!(
        primitives["id"],
        PrimitiveValue::Text("1ea47840-aa39-5aa8-b4a2-cd203eb6662a".to_string())
    );
    assert_eq!(
        primitives["value"],
        PrimitiveValue::Text("198.51.100.0/24".to_string())
    );
}

#[test]
fn it_carries_the_id_of_its_own_value() {
    let ipv4_network = Ipv4NetworkMother::random();

    let event = create_ipv4_network_deleted_event(&ipv4_network).expect("event must be created");

    assert_eq!(event.id(), ipv4_network.id());
    assert_eq!(*event.id(), Ipv4NetworkId::from_value(event.value()));
}
