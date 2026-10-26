use kernel::ipv6_network::domain::value_objects::ipv6_network_id::Ipv6NetworkId;

use crate::src::ipv6_network::domain::value_objects::mothers::ipv6_network_value_mother::Ipv6NetworkValueMother;

pub struct Ipv6NetworkIdMother;

#[allow(dead_code)]
impl Ipv6NetworkIdMother {
    pub fn create(value: &str) -> Ipv6NetworkId {
        Ipv6NetworkId::from_value(&Ipv6NetworkValueMother::create(value))
    }

    pub fn random() -> Ipv6NetworkId {
        Ipv6NetworkId::from_value(&Ipv6NetworkValueMother::random())
    }
}
