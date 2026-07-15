use kernel::url_source::domain::value_objects::url_source_format::UrlSourceFormat;

pub struct UrlSourceFormatMother;

#[allow(dead_code)]
impl UrlSourceFormatMother {
    pub fn plain() -> UrlSourceFormat {
        UrlSourceFormat::Plain
    }

    pub fn csv() -> UrlSourceFormat {
        UrlSourceFormat::Csv
    }

    pub fn json() -> UrlSourceFormat {
        UrlSourceFormat::Json
    }

    pub fn stix() -> UrlSourceFormat {
        UrlSourceFormat::Stix
    }

    pub fn random() -> UrlSourceFormat {
        UrlSourceFormat::Plain
    }
}
