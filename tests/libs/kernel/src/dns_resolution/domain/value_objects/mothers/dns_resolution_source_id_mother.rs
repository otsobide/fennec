use kernel::dns_resolution::domain::value_objects::dns_resolution_source_id::DnsResolutionSourceId;

pub struct DnsResolutionSourceIdMother;

#[allow(dead_code)]
impl DnsResolutionSourceIdMother {
    pub fn create(value: &str) -> DnsResolutionSourceId {
        DnsResolutionSourceId::new(value)
            .expect("DnsResolutionSourceIdMother::create received an invalid id")
    }

    pub fn random() -> DnsResolutionSourceId {
        DnsResolutionSourceId::generate()
    }
}
