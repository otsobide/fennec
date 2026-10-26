use std::time::{Duration, UNIX_EPOCH};

use shared_valueobject::domain::timestamps::truncate_to_microseconds::truncate_to_microseconds;

#[test]
fn it_drops_the_nanoseconds_after_the_epoch() {
    let instant = UNIX_EPOCH + Duration::from_nanos(1_700_000_000_123_456_789);

    assert_eq!(
        truncate_to_microseconds(instant),
        UNIX_EPOCH + Duration::from_micros(1_700_000_000_123_456)
    );
}

#[test]
fn it_truncates_towards_the_epoch_before_it() {
    let instant = UNIX_EPOCH - Duration::from_nanos(1_500);

    assert_eq!(
        truncate_to_microseconds(instant),
        UNIX_EPOCH - Duration::from_micros(1)
    );
}

#[test]
fn it_keeps_whole_microseconds_unchanged() {
    let instant = UNIX_EPOCH + Duration::from_micros(42);

    assert_eq!(truncate_to_microseconds(instant), instant);
}

#[test]
fn it_is_idempotent() {
    let instant = UNIX_EPOCH + Duration::from_nanos(987_654_321);

    assert_eq!(
        truncate_to_microseconds(truncate_to_microseconds(instant)),
        truncate_to_microseconds(instant)
    );
}
