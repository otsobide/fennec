use kernel::email_address::domain::value_objects::email_address_id::EmailAddressId;

use crate::src::email_address::domain::value_objects::mothers::email_address_value_mother::EmailAddressValueMother;

pub struct EmailAddressIdMother;

#[allow(dead_code)]
impl EmailAddressIdMother {
    pub fn create(value: &str) -> EmailAddressId {
        EmailAddressId::from_value(&EmailAddressValueMother::create(value))
    }

    pub fn random() -> EmailAddressId {
        EmailAddressId::from_value(&EmailAddressValueMother::random())
    }
}
