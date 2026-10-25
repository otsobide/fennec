use std::time::{Duration, UNIX_EPOCH};

use shared_valueobject::domain::timestamps::microsecond_timestamp::{
    MicrosecondTimestamp, MAX_UNIX_MICROS,
};

#[test]
fn it_truncates_nanoseconds_to_microseconds() {
    let instant = UNIX_EPOCH + Duration::from_nanos(1_700_000_000_123_456_789);

    let timestamp = MicrosecondTimestamp::from_system_time(instant).expect("valid instant");

    assert_eq!(
        timestamp.value(),
        UNIX_EPOCH + Duration::from_micros(1_700_000_000_123_456)
    );
}

#[test]
fn it_compares_equal_after_a_round_trip_through_microseconds() {
    let original = MicrosecondTimestamp::now();

    let reloaded =
        MicrosecondTimestamp::from_unix_micros(original.unix_micros()).expect("valid instant");

    assert_eq!(original, reloaded);
}

#[test]
fn it_accepts_the_unix_epoch() {
    let timestamp =
        MicrosecondTimestamp::from_system_time(UNIX_EPOCH).expect("the epoch is accepted");

    assert_eq!(timestamp.unix_micros(), 0);
}

#[test]
fn it_rejects_instants_before_the_unix_epoch() {
    let error = MicrosecondTimestamp::from_system_time(UNIX_EPOCH - Duration::from_secs(1))
        .expect_err("pre-epoch instants must be rejected");

    assert!(error.message().contains("before 1970"));
}

#[test]
fn it_accepts_the_last_microsecond_of_year_9999() {
    let timestamp =
        MicrosecondTimestamp::from_unix_micros(MAX_UNIX_MICROS).expect("the limit is accepted");

    assert_eq!(timestamp.unix_seconds(), 253_402_300_799);
}

#[test]
fn it_rejects_instants_after_year_9999() {
    let error = MicrosecondTimestamp::from_unix_micros(MAX_UNIX_MICROS + 1)
        .expect_err("one microsecond over the limit must be rejected");

    assert!(error.message().contains("after 9999"));
}

#[test]
fn it_rejects_system_times_far_beyond_year_9999() {
    let error =
        MicrosecondTimestamp::from_system_time(UNIX_EPOCH + Duration::from_secs(300_000_000_000))
            .expect_err("instants beyond year 9999 must be rejected");

    assert!(error.message().contains("after 9999"));
}

#[test]
fn it_reports_whole_seconds_since_the_epoch() {
    let timestamp =
        MicrosecondTimestamp::from_unix_micros(1_700_000_000_999_999).expect("valid instant");

    assert_eq!(timestamp.unix_seconds(), 1_700_000_000);
}

#[test]
fn it_orders_timestamps_chronologically() {
    let earlier = MicrosecondTimestamp::from_unix_micros(1).expect("valid instant");
    let later = MicrosecondTimestamp::from_unix_micros(2).expect("valid instant");

    assert!(earlier < later);
}
