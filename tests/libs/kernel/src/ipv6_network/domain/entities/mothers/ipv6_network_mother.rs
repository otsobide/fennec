use kernel::ipv6_network::domain::entities::ipv6_network::Ipv6Network;
use kernel::ipv6_network::domain::value_objects::ipv6_network_created_at::Ipv6NetworkCreatedAt;

use crate::src::ipv6_network::domain::value_objects::mothers::ipv6_network_value_mother::Ipv6NetworkValueMother;

pub struct Ipv6NetworkMother;

#[allow(dead_code)]
impl Ipv6NetworkMother {
    pub fn create(value: &str) -> Ipv6Network {
        Ipv6Network::new(
            Ipv6NetworkValueMother::create(value),
            Ipv6NetworkCreatedAt::now(),
        )
    }

    pub fn random() -> Ipv6Network {
        Ipv6Network::new(
            Ipv6NetworkValueMother::random(),
            Ipv6NetworkCreatedAt::now(),
        )
    }
}
