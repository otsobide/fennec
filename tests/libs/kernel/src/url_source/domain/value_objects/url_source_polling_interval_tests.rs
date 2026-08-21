use kernel::url_source::domain::value_objects::url_source_polling_interval::UrlSourcePollingInterval;

#[test]
fn it_accepts_the_smallest_positive_interval() {
    let interval = UrlSourcePollingInterval::from_seconds(1).expect("one second is valid");

    assert_eq!(interval.seconds(), 1);
}

#[test]
fn it_rejects_a_zero_interval() {
    let error =
        UrlSourcePollingInterval::from_seconds(0).expect_err("zero seconds must be rejected");

    assert!(error.message().contains("greater than 0"));
}

#[test]
fn it_renders_the_interval_in_seconds() {
    let interval = UrlSourcePollingInterval::from_seconds(300).expect("valid interval");

    assert_eq!(interval.to_string(), "300s");
}
