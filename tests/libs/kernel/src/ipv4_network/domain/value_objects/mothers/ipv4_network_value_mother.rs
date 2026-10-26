use std::net::Ipv4Addr;

use kernel::ipv4_network::domain::value_objects::ipv4_network_value::Ipv4NetworkValue;

pub struct Ipv4NetworkValueMother;

#[allow(dead_code)]
impl Ipv4NetworkValueMother {
    pub fn create(value: &str) -> Ipv4NetworkValue {
        Ipv4NetworkValue::new(value)
            .expect("Ipv4NetworkValueMother::create received an invalid network")
    }

    pub fn random() -> Ipv4NetworkValue {
        let prefix_length = 8 + rand::random::<u8>() % 24;
        let mask = u32::MAX << (32 - u32::from(prefix_length));
        let address = Ipv4Addr::from(rand::random::<u32>() & mask);
        Self::create(&format!("{address}/{prefix_length}"))
    }
}
