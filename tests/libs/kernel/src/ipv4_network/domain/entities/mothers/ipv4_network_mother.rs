use kernel::ipv4_network::domain::entities::ipv4_network::Ipv4Network;
use kernel::ipv4_network::domain::value_objects::ipv4_network_created_at::Ipv4NetworkCreatedAt;

use crate::src::ipv4_network::domain::value_objects::mothers::ipv4_network_value_mother::Ipv4NetworkValueMother;

pub struct Ipv4NetworkMother;

#[allow(dead_code)]
impl Ipv4NetworkMother {
    pub fn create(value: &str) -> Ipv4Network {
        Ipv4Network::new(
            Ipv4NetworkValueMother::create(value),
            Ipv4NetworkCreatedAt::now(),
        )
    }

    pub fn random() -> Ipv4Network {
        Ipv4Network::new(
            Ipv4NetworkValueMother::random(),
            Ipv4NetworkCreatedAt::now(),
        )
    }
}
