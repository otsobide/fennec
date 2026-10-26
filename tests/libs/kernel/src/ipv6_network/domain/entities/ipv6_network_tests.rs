use kernel::ipv6_network::domain::entities::ipv6_network::Ipv6Network;
use kernel::ipv6_network::domain::value_objects::ipv6_network_created_at::Ipv6NetworkCreatedAt;
use kernel::ipv6_network::domain::value_objects::ipv6_network_id::Ipv6NetworkId;

use crate::src::ipv6_network::domain::value_objects::mothers::ipv6_network_value_mother::Ipv6NetworkValueMother;

#[test]
fn it_derives_its_id_from_its_value() {
    let value = Ipv6NetworkValueMother::create("2001:db8::/32");

    let ipv6_network = Ipv6Network::new(value, Ipv6NetworkCreatedAt::now());

    assert_eq!(*ipv6_network.id(), Ipv6NetworkId::from_value(&value));
    assert_eq!(
        ipv6_network.id().stix_id(),
        "ipv6-addr--8da31feb-e0f9-54c3-ab30-bd8ccea361dd"
    );
}

#[test]
fn it_keeps_its_value_and_creation_time() {
    let value = Ipv6NetworkValueMother::random();
    let created_at = Ipv6NetworkCreatedAt::now();

    let ipv6_network = Ipv6Network::new(value, created_at);

    assert_eq!(*ipv6_network.value(), value);
    assert_eq!(*ipv6_network.created_at(), created_at);
}
