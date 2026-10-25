use std::net::Ipv4Addr;

use kernel::ipv4_address::domain::value_objects::ipv4_address_value::Ipv4AddressValue;

pub struct Ipv4AddressValueMother;

#[allow(dead_code)]
impl Ipv4AddressValueMother {
    pub fn create(value: &str) -> Ipv4AddressValue {
        Ipv4AddressValue::new(value)
            .expect("Ipv4AddressValueMother::create received an invalid address")
    }

    pub fn random() -> Ipv4AddressValue {
        Ipv4AddressValue::from_ipv4_addr(Ipv4Addr::from(rand::random::<u32>()))
    }
}
