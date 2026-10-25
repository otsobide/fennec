use kernel::ipv4_address::domain::entities::ipv4_address::Ipv4Address;
use kernel::ipv4_address::domain::value_objects::ipv4_address_created_at::Ipv4AddressCreatedAt;

use crate::src::ipv4_address::domain::value_objects::mothers::ipv4_address_value_mother::Ipv4AddressValueMother;

pub struct Ipv4AddressMother;

#[allow(dead_code)]
impl Ipv4AddressMother {
    pub fn create(value: &str) -> Ipv4Address {
        Ipv4Address::new(
            Ipv4AddressValueMother::create(value),
            Ipv4AddressCreatedAt::now(),
        )
    }

    pub fn random() -> Ipv4Address {
        Ipv4Address::new(
            Ipv4AddressValueMother::random(),
            Ipv4AddressCreatedAt::now(),
        )
    }
}
