use kernel::ipv6_address::domain::entities::ipv6_address::Ipv6Address;
use kernel::ipv6_address::domain::value_objects::ipv6_address_created_at::Ipv6AddressCreatedAt;

use crate::src::ipv6_address::domain::value_objects::mothers::ipv6_address_value_mother::Ipv6AddressValueMother;

pub struct Ipv6AddressMother;

#[allow(dead_code)]
impl Ipv6AddressMother {
    pub fn create(value: &str) -> Ipv6Address {
        Ipv6Address::new(
            Ipv6AddressValueMother::create(value),
            Ipv6AddressCreatedAt::now(),
        )
    }

    pub fn random() -> Ipv6Address {
        Ipv6Address::new(
            Ipv6AddressValueMother::random(),
            Ipv6AddressCreatedAt::now(),
        )
    }
}
