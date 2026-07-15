use kernel::url_source::domain::value_objects::url_source_url::UrlSourceUrl;
use uuid::Uuid;

pub struct UrlSourceUrlMother;

#[allow(dead_code)]
impl UrlSourceUrlMother {
    pub fn create(value: impl Into<String>) -> UrlSourceUrl {
        UrlSourceUrl::new(value).expect("UrlSourceUrlMother::create received invalid url")
    }

    pub fn random() -> UrlSourceUrl {
        UrlSourceUrl::new(format!("https://feeds.example.com/{}", Uuid::new_v4()))
            .expect("random url must be valid")
    }
}
