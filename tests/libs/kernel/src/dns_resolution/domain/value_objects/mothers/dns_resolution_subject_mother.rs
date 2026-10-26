use kernel::dns_resolution::domain::value_objects::dns_resolution_subject::DnsResolutionSubject;
use uuid::Uuid;

pub struct DnsResolutionSubjectMother;

#[allow(dead_code)]
impl DnsResolutionSubjectMother {
    pub fn create(domain_name: &str) -> DnsResolutionSubject {
        DnsResolutionSubject::domain_name(domain_name)
            .expect("DnsResolutionSubjectMother::create received an invalid domain name")
    }

    pub fn random() -> DnsResolutionSubject {
        Self::create(&format!("d{}.example", Uuid::new_v4().simple()))
    }
}
