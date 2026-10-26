use std::time::{Duration, UNIX_EPOCH};

use kernel::url_source::domain::value_objects::url_source_created_at::UrlSourceCreatedAt;

#[test]
fn it_truncates_the_current_instant_to_microseconds() {
    let nanos = UrlSourceCreatedAt::now()
        .value()
        .duration_since(UNIX_EPOCH)
        .expect("after the epoch")
        .subsec_nanos();

    assert_eq!(nanos % 1_000, 0);
}

#[test]
fn it_truncates_an_explicit_instant_to_microseconds() {
    let instant = UNIX_EPOCH + Duration::from_nanos(1_700_000_000_123_456_789);

    assert_eq!(
        UrlSourceCreatedAt::from_system_time(instant).value(),
        UNIX_EPOCH + Duration::from_micros(1_700_000_000_123_456)
    );
}

#[test]
fn it_compares_equal_after_a_microsecond_round_trip() {
    let original = UrlSourceCreatedAt::now();

    assert_eq!(
        UrlSourceCreatedAt::from_system_time(original.value()),
        original
    );
}
