use std::time::{Duration, UNIX_EPOCH};

use kernel::sighting::domain::value_objects::sighting_observed_at::SightingObservedAt;

#[test]
fn it_builds_the_instant_from_unix_seconds() {
    let observed_at = SightingObservedAt::from_unix_seconds(1_750_000_000).expect("valid instant");

    assert_eq!(
        observed_at.value(),
        UNIX_EPOCH + Duration::from_secs(1_750_000_000)
    );
}

#[test]
fn it_accepts_the_last_second_of_year_9999() {
    assert!(SightingObservedAt::from_unix_seconds(253_402_300_799).is_ok());
}

#[test]
fn it_rejects_the_first_second_of_year_10000() {
    let error = SightingObservedAt::from_unix_seconds(253_402_300_800)
        .expect_err("year 10000 is out of range");

    assert!(error.message().contains("must not be after 9999"));
}

#[test]
fn it_rejects_values_that_would_overflow() {
    let error = SightingObservedAt::from_unix_seconds(u64::MAX).expect_err("must not overflow");

    assert!(error.message().contains("must not be after 9999"));
}
