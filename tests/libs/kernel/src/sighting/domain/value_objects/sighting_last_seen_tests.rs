use std::time::{Duration, UNIX_EPOCH};

use kernel::sighting::domain::value_objects::sighting_last_seen::SightingLastSeen;

#[test]
fn it_truncates_an_explicit_instant_to_microseconds() {
    let instant = UNIX_EPOCH + Duration::from_nanos(1_700_000_000_123_456_789);

    assert_eq!(
        SightingLastSeen::from_system_time(instant).value(),
        UNIX_EPOCH + Duration::from_micros(1_700_000_000_123_456)
    );
}

#[test]
fn it_compares_equal_after_a_microsecond_round_trip() {
    let original = SightingLastSeen::from_system_time(
        UNIX_EPOCH + Duration::from_nanos(1_750_000_000_987_654_321),
    );

    assert_eq!(
        SightingLastSeen::from_system_time(original.value()),
        original
    );
}
