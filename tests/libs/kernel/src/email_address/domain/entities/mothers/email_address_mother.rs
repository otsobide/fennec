use kernel::email_address::domain::entities::email_address::EmailAddress;
use kernel::email_address::domain::value_objects::email_address_created_at::EmailAddressCreatedAt;

use crate::src::email_address::domain::value_objects::mothers::email_address_value_mother::EmailAddressValueMother;

pub struct EmailAddressMother;

#[allow(dead_code)]
impl EmailAddressMother {
    pub fn create(value: &str) -> EmailAddress {
        EmailAddress::new(
            EmailAddressValueMother::create(value),
            EmailAddressCreatedAt::now(),
        )
    }

    pub fn random() -> EmailAddress {
        EmailAddress::new(
            EmailAddressValueMother::random(),
            EmailAddressCreatedAt::now(),
        )
    }
}
