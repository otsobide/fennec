use kernel::domain_name::domain::entities::domain_name::DomainName;
use kernel::domain_name::domain::value_objects::domain_name_created_at::DomainNameCreatedAt;

use crate::src::domain_name::domain::value_objects::mothers::domain_name_value_mother::DomainNameValueMother;

pub struct DomainNameMother;

#[allow(dead_code)]
impl DomainNameMother {
    pub fn create(value: &str) -> DomainName {
        DomainName::new(
            DomainNameValueMother::create(value),
            DomainNameCreatedAt::now(),
        )
    }

    pub fn random() -> DomainName {
        DomainName::new(DomainNameValueMother::random(), DomainNameCreatedAt::now())
    }
}
