use std::time::{Duration, SystemTime, UNIX_EPOCH};

use kernel::source::domain::value_objects::source_updated_at::SourceUpdatedAt;

#[test]
fn it_truncates_the_current_instant_to_microseconds() {
    let nanos = SourceUpdatedAt::now()
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
        SourceUpdatedAt::from_system_time(instant).value(),
        UNIX_EPOCH + Duration::from_micros(1_700_000_000_123_456)
    );
}

#[test]
fn it_compares_equal_after_a_microsecond_round_trip() {
    let original = SourceUpdatedAt::now();

    let reloaded = SourceUpdatedAt::from_system_time(original.value());

    assert_eq!(original, reloaded);
    assert!(original.value() <= SystemTime::now());
}
