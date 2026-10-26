use std::net::{Ipv4Addr, Ipv6Addr};

use kernel::dns_resolution::domain::value_objects::dns_resolution_answer::DnsResolutionAnswer;

pub struct DnsResolutionAnswerMother;

#[allow(dead_code)]
impl DnsResolutionAnswerMother {
    pub fn ipv4(address: &str) -> DnsResolutionAnswer {
        DnsResolutionAnswer::ipv4_address(address)
            .expect("DnsResolutionAnswerMother::ipv4 received an invalid address")
    }

    pub fn ipv6(address: &str) -> DnsResolutionAnswer {
        DnsResolutionAnswer::ipv6_address(address)
            .expect("DnsResolutionAnswerMother::ipv6 received an invalid address")
    }

    pub fn random() -> DnsResolutionAnswer {
        if rand::random::<bool>() {
            DnsResolutionAnswer::Ipv4Address(Ipv4Addr::from(rand::random::<u32>()))
        } else {
            DnsResolutionAnswer::Ipv6Address(Ipv6Addr::from(rand::random::<u128>()))
        }
    }
}
