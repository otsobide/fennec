use kernel::ipv4_network::domain::entities::ipv4_network::Ipv4Network;
use kernel::ipv4_network::domain::value_objects::ipv4_network_created_at::Ipv4NetworkCreatedAt;
use kernel::ipv4_network::domain::value_objects::ipv4_network_id::Ipv4NetworkId;

use crate::src::ipv4_network::domain::value_objects::mothers::ipv4_network_value_mother::Ipv4NetworkValueMother;

#[test]
fn it_derives_its_id_from_its_value() {
    let value = Ipv4NetworkValueMother::create("198.51.100.0/24");

    let ipv4_network = Ipv4Network::new(value, Ipv4NetworkCreatedAt::now());

    assert_eq!(*ipv4_network.id(), Ipv4NetworkId::from_value(&value));
    assert_eq!(
        ipv4_network.id().stix_id(),
        "ipv4-addr--1ea47840-aa39-5aa8-b4a2-cd203eb6662a"
    );
}

#[test]
fn it_keeps_its_value_and_creation_time() {
    let value = Ipv4NetworkValueMother::random();
    let created_at = Ipv4NetworkCreatedAt::now();

    let ipv4_network = Ipv4Network::new(value, created_at);

    assert_eq!(*ipv4_network.value(), value);
    assert_eq!(*ipv4_network.created_at(), created_at);
}
