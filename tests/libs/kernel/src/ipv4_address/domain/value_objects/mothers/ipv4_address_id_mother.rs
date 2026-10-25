use kernel::ipv4_address::domain::value_objects::ipv4_address_id::Ipv4AddressId;

use crate::src::ipv4_address::domain::value_objects::mothers::ipv4_address_value_mother::Ipv4AddressValueMother;

pub struct Ipv4AddressIdMother;

#[allow(dead_code)]
impl Ipv4AddressIdMother {
    pub fn random() -> Ipv4AddressId {
        Ipv4AddressId::from_value(&Ipv4AddressValueMother::random())
    }
}
