use kernel::ipv4_network::domain::value_objects::ipv4_network_id::Ipv4NetworkId;

use crate::src::ipv4_network::domain::value_objects::mothers::ipv4_network_value_mother::Ipv4NetworkValueMother;

pub struct Ipv4NetworkIdMother;

#[allow(dead_code)]
impl Ipv4NetworkIdMother {
    pub fn create(value: &str) -> Ipv4NetworkId {
        Ipv4NetworkId::from_value(&Ipv4NetworkValueMother::create(value))
    }

    pub fn random() -> Ipv4NetworkId {
        Ipv4NetworkId::from_value(&Ipv4NetworkValueMother::random())
    }
}
