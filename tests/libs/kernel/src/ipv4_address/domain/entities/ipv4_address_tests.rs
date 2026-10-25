use kernel::ipv4_address::domain::entities::ipv4_address::Ipv4Address;
use kernel::ipv4_address::domain::value_objects::ipv4_address_created_at::Ipv4AddressCreatedAt;
use kernel::ipv4_address::domain::value_objects::ipv4_address_id::Ipv4AddressId;

use crate::src::ipv4_address::domain::value_objects::mothers::ipv4_address_value_mother::Ipv4AddressValueMother;

#[test]
fn it_derives_its_id_from_its_value() {
    let value = Ipv4AddressValueMother::create("89.19.209.154");

    let ipv4_address = Ipv4Address::new(value, Ipv4AddressCreatedAt::now());

    assert_eq!(*ipv4_address.id(), Ipv4AddressId::from_value(&value));
    assert_eq!(
        ipv4_address.id().stix_id(),
        "ipv4-addr--abfb438a-8baf-5f54-b19e-b85ee74b3ed7"
    );
}

#[test]
fn it_keeps_its_value_and_creation_time() {
    let value = Ipv4AddressValueMother::random();
    let created_at = Ipv4AddressCreatedAt::now();

    let ipv4_address = Ipv4Address::new(value, created_at);

    assert_eq!(*ipv4_address.value(), value);
    assert_eq!(*ipv4_address.created_at(), created_at);
}
