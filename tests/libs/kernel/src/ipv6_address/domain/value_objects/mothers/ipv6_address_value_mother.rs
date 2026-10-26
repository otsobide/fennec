use std::net::Ipv6Addr;

use kernel::ipv6_address::domain::value_objects::ipv6_address_value::Ipv6AddressValue;

pub struct Ipv6AddressValueMother;

#[allow(dead_code)]
impl Ipv6AddressValueMother {
    pub fn create(value: &str) -> Ipv6AddressValue {
        Ipv6AddressValue::new(value)
            .expect("Ipv6AddressValueMother::create received an invalid address")
    }

    pub fn random() -> Ipv6AddressValue {
        Ipv6AddressValue::from_ipv6_addr(Ipv6Addr::from(rand::random::<u128>()))
    }
}
