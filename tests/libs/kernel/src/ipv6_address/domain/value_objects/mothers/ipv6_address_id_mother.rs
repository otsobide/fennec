use kernel::ipv6_address::domain::value_objects::ipv6_address_id::Ipv6AddressId;

use crate::src::ipv6_address::domain::value_objects::mothers::ipv6_address_value_mother::Ipv6AddressValueMother;

pub struct Ipv6AddressIdMother;

#[allow(dead_code)]
impl Ipv6AddressIdMother {
    pub fn create(value: &str) -> Ipv6AddressId {
        Ipv6AddressId::from_value(&Ipv6AddressValueMother::create(value))
    }

    pub fn random() -> Ipv6AddressId {
        Ipv6AddressId::from_value(&Ipv6AddressValueMother::random())
    }
}
