use config::config_entry::domain::value_objects::config_value::ConfigValue;
use uuid::Uuid;

pub struct ConfigValueMother;

#[allow(dead_code)]
impl ConfigValueMother {
    pub fn create(value: impl Into<String>) -> ConfigValue {
        ConfigValue::new(value).expect("mother must build a valid value object")
    }

    pub fn random() -> ConfigValue {
        ConfigValue::new(Uuid::new_v4().to_string())
            .expect("mother must build a valid value object")
    }
}
