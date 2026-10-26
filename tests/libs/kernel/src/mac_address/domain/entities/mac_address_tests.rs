use kernel::mac_address::domain::entities::mac_address::MacAddress;
use kernel::mac_address::domain::value_objects::mac_address_created_at::MacAddressCreatedAt;
use kernel::mac_address::domain::value_objects::mac_address_id::MacAddressId;

use crate::src::mac_address::domain::value_objects::mothers::mac_address_value_mother::MacAddressValueMother;

#[test]
fn it_derives_its_id_from_its_value() {
    let value = MacAddressValueMother::create("d2:fb:49:24:37:18");

    let mac_address = MacAddress::new(value, MacAddressCreatedAt::now());

    assert_eq!(*mac_address.id(), MacAddressId::from_value(&value));
    assert_eq!(
        mac_address.id().stix_id(),
        "mac-addr--757b1725-9903-54f5-a855-1240691d7659"
    );
}

#[test]
fn it_keeps_its_value_and_creation_time() {
    let value = MacAddressValueMother::random();
    let created_at = MacAddressCreatedAt::now();

    let mac_address = MacAddress::new(value, created_at);

    assert_eq!(*mac_address.value(), value);
    assert_eq!(*mac_address.created_at(), created_at);
}
