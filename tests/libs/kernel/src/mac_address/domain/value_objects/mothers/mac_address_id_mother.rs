use kernel::mac_address::domain::value_objects::mac_address_id::MacAddressId;

use crate::src::mac_address::domain::value_objects::mothers::mac_address_value_mother::MacAddressValueMother;

pub struct MacAddressIdMother;

#[allow(dead_code)]
impl MacAddressIdMother {
    pub fn create(value: &str) -> MacAddressId {
        MacAddressId::from_value(&MacAddressValueMother::create(value))
    }

    pub fn random() -> MacAddressId {
        MacAddressId::from_value(&MacAddressValueMother::random())
    }
}
