use std::net::Ipv6Addr;

use kernel::ipv6_network::domain::value_objects::ipv6_network_value::Ipv6NetworkValue;

pub struct Ipv6NetworkValueMother;

#[allow(dead_code)]
impl Ipv6NetworkValueMother {
    pub fn create(value: &str) -> Ipv6NetworkValue {
        Ipv6NetworkValue::new(value)
            .expect("Ipv6NetworkValueMother::create received an invalid network")
    }

    pub fn random() -> Ipv6NetworkValue {
        let prefix_length = 16 + rand::random::<u8>() % 112;
        let mask = u128::MAX << (128 - u32::from(prefix_length));
        let address = Ipv6Addr::from(rand::random::<u128>() & mask);
        Self::create(&format!("{address}/{prefix_length}"))
    }
}
