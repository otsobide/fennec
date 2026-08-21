use kernel::ioc::domain::entities::ioc::Ioc;
use kernel::ioc::domain::value_objects::ioc_created_at::IocCreatedAt;
use kernel::ioc::domain::value_objects::ioc_updated_at::IocUpdatedAt;

use crate::src::ioc::domain::value_objects::mothers::ioc_id_mother::IocIdMother;
use crate::src::ioc::domain::value_objects::mothers::ioc_type_mother::IocTypeMother;
use crate::src::ioc::domain::value_objects::mothers::ioc_value_mother::IocValueMother;

pub struct IocMother;

#[allow(dead_code)]
impl IocMother {
    pub fn random() -> Ioc {
        let created_at = IocCreatedAt::now();
        let updated_at = IocUpdatedAt::from_system_time(created_at.value());
        Ioc::new(
            IocIdMother::random(),
            IocTypeMother::random(),
            IocValueMother::random(),
            created_at,
            updated_at,
        )
    }
}
