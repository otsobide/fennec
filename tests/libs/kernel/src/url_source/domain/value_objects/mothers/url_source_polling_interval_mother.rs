use kernel::url_source::domain::value_objects::url_source_polling_interval::UrlSourcePollingInterval;

pub struct UrlSourcePollingIntervalMother;

#[allow(dead_code)]
impl UrlSourcePollingIntervalMother {
    pub fn create(seconds: u32) -> UrlSourcePollingInterval {
        UrlSourcePollingInterval::from_seconds(seconds)
            .expect("UrlSourcePollingIntervalMother::create received zero")
    }

    pub fn random() -> UrlSourcePollingInterval {
        UrlSourcePollingInterval::from_seconds(3600).expect("3600 is positive")
    }
}
