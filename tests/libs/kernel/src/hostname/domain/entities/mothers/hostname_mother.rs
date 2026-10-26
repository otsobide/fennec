use kernel::hostname::domain::entities::hostname::Hostname;
use kernel::hostname::domain::value_objects::hostname_created_at::HostnameCreatedAt;

use crate::src::hostname::domain::value_objects::mothers::hostname_value_mother::HostnameValueMother;

pub struct HostnameMother;

#[allow(dead_code)]
impl HostnameMother {
    pub fn create(value: &str) -> Hostname {
        Hostname::new(HostnameValueMother::create(value), HostnameCreatedAt::now())
    }

    pub fn random() -> Hostname {
        Hostname::new(HostnameValueMother::random(), HostnameCreatedAt::now())
    }
}
