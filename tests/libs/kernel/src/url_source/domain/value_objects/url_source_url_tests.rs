use kernel::url_source::domain::value_objects::url_source_url::{UrlSourceUrl, MAX_LENGTH};

#[test]
fn it_accepts_http_and_https_urls() {
    for raw in [
        "http://example.test/feed.txt",
        "https://example.test/feed.txt",
    ] {
        assert!(UrlSourceUrl::new(raw).is_ok());
    }
}

#[test]
fn it_trims_surrounding_whitespace() {
    let url = UrlSourceUrl::new("  https://example.test/feed.txt  ").expect("valid url");

    assert_eq!(url.value(), "https://example.test/feed.txt");
}

#[test]
fn it_rejects_an_empty_url() {
    let error = UrlSourceUrl::new("   ").expect_err("an empty url must be rejected");

    assert!(error.message().contains("must not be empty"));
}

#[test]
fn it_rejects_an_unsupported_scheme() {
    let error = UrlSourceUrl::new("ftp://example.test/feed.txt")
        .expect_err("a non-http scheme must be rejected");

    assert!(error.message().contains("http://"));
}

#[test]
fn it_accepts_exactly_the_maximum_length() {
    let padding = "a".repeat(MAX_LENGTH - "https://".len());
    let url = UrlSourceUrl::new(format!("https://{padding}")).expect("the limit itself is valid");

    assert_eq!(url.value().chars().count(), MAX_LENGTH);
}

#[test]
fn it_rejects_one_character_over_the_maximum_length() {
    let padding = "a".repeat(MAX_LENGTH - "https://".len() + 1);
    let error = UrlSourceUrl::new(format!("https://{padding}"))
        .expect_err("one character over the limit must be rejected");

    assert!(error.message().contains("at most"));
}
