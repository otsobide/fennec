use kernel::url_source::domain::entities::url_source::UrlSource;
use kernel::url_source::domain::value_objects::url_source_created_at::UrlSourceCreatedAt;
use kernel::url_source::domain::value_objects::url_source_updated_at::UrlSourceUpdatedAt;

use crate::src::url_source::domain::value_objects::mothers::url_source_format_mother::UrlSourceFormatMother;
use crate::src::url_source::domain::value_objects::mothers::url_source_id_mother::UrlSourceIdMother;
use crate::src::url_source::domain::value_objects::mothers::url_source_polling_interval_mother::UrlSourcePollingIntervalMother;
use crate::src::url_source::domain::value_objects::mothers::url_source_url_mother::UrlSourceUrlMother;

pub struct UrlSourceMother;

#[allow(dead_code)]
impl UrlSourceMother {
    pub fn random() -> UrlSource {
        let created_at = UrlSourceCreatedAt::now();
        let updated_at = UrlSourceUpdatedAt::from_system_time(created_at.value());
        UrlSource::new(
            UrlSourceIdMother::random(),
            UrlSourceUrlMother::random(),
            UrlSourceFormatMother::random(),
            UrlSourcePollingIntervalMother::random(),
            created_at,
            updated_at,
        )
    }
}
