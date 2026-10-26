use kernel::mac_address::domain::entities::mac_address::MacAddress;
use kernel::mac_address::domain::value_objects::mac_address_created_at::MacAddressCreatedAt;

use crate::src::mac_address::domain::value_objects::mothers::mac_address_value_mother::MacAddressValueMother;

pub struct MacAddressMother;

#[allow(dead_code)]
impl MacAddressMother {
    pub fn create(value: &str) -> MacAddress {
        MacAddress::new(
            MacAddressValueMother::create(value),
            MacAddressCreatedAt::now(),
        )
    }

    pub fn random() -> MacAddress {
        MacAddress::new(MacAddressValueMother::random(), MacAddressCreatedAt::now())
    }
}
