use kernel::ipv6_address::domain::entities::ipv6_address::Ipv6Address;
use kernel::ipv6_address::domain::value_objects::ipv6_address_created_at::Ipv6AddressCreatedAt;
use kernel::ipv6_address::domain::value_objects::ipv6_address_id::Ipv6AddressId;

use crate::src::ipv6_address::domain::value_objects::mothers::ipv6_address_value_mother::Ipv6AddressValueMother;

#[test]
fn it_derives_its_id_from_its_value() {
    let value = Ipv6AddressValueMother::create("2001:db8:85a3::8a2e:370:7334");

    let ipv6_address = Ipv6Address::new(value, Ipv6AddressCreatedAt::now());

    assert_eq!(*ipv6_address.id(), Ipv6AddressId::from_value(&value));
    assert_eq!(
        ipv6_address.id().stix_id(),
        "ipv6-addr--f5344921-72d2-5619-945d-8757bca4b7cd"
    );
}

#[test]
fn it_keeps_its_value_and_creation_time() {
    let value = Ipv6AddressValueMother::random();
    let created_at = Ipv6AddressCreatedAt::now();

    let ipv6_address = Ipv6Address::new(value, created_at);

    assert_eq!(*ipv6_address.value(), value);
    assert_eq!(*ipv6_address.created_at(), created_at);
}
