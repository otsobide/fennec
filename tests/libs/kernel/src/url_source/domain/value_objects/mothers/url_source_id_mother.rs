use kernel::url_source::domain::value_objects::url_source_id::UrlSourceId;
use uuid::Uuid;

pub struct UrlSourceIdMother;

#[allow(dead_code)]
impl UrlSourceIdMother {
    pub fn create(value: Uuid) -> UrlSourceId {
        UrlSourceId::from_uuid(value)
    }

    pub fn random() -> UrlSourceId {
        UrlSourceId::from_uuid(Uuid::new_v4())
    }
}
