use kernel::source::domain::value_objects::source_description::SourceDescription;
use uuid::Uuid;

pub struct SourceDescriptionMother;

#[allow(dead_code)]
impl SourceDescriptionMother {
    pub fn create(value: impl Into<String>) -> SourceDescription {
        SourceDescription::new(value).expect("mother must build a valid value object")
    }

    pub fn random() -> SourceDescription {
        SourceDescription::new(format!("description-{}", Uuid::new_v4()))
            .expect("mother must build a valid value object")
    }
}
