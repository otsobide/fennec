use std::time::{Duration, UNIX_EPOCH};

use kernel::dns_resolution::domain::value_objects::dns_resolution_created_at::DnsResolutionCreatedAt;
use kernel::dns_resolution::domain::value_objects::dns_resolution_updated_at::DnsResolutionUpdatedAt;
use shared_valueobject::domain::timestamps::microsecond_timestamp::MAX_UNIX_MICROS;

#[test]
fn it_records_the_current_instant_with_microsecond_precision() {
    let nanos = DnsResolutionUpdatedAt::now()
        .value()
        .duration_since(UNIX_EPOCH)
        .expect("after the epoch")
        .subsec_nanos();

    assert_eq!(nanos % 1_000, 0);
}

#[test]
fn it_starts_at_the_creation_instant() {
    let created_at = DnsResolutionCreatedAt::now();

    let updated_at = DnsResolutionUpdatedAt::from_created_at(&created_at);

    assert_eq!(updated_at.unix_micros(), created_at.unix_micros());
    assert_eq!(updated_at.value(), created_at.value());
}

#[test]
fn it_round_trips_through_unix_microseconds() {
    let updated_at = DnsResolutionUpdatedAt::now();

    let reloaded =
        DnsResolutionUpdatedAt::from_unix_micros(updated_at.unix_micros()).expect("in range");

    assert_eq!(reloaded, updated_at);
}

#[test]
fn it_exposes_its_instant_in_every_form() {
    let updated_at =
        DnsResolutionUpdatedAt::from_unix_micros(1_700_000_000_123_456).expect("in range");

    assert_eq!(updated_at.unix_micros(), 1_700_000_000_123_456);
    assert_eq!(
        updated_at.value(),
        UNIX_EPOCH + Duration::from_micros(1_700_000_000_123_456)
    );
    assert_eq!(updated_at.timestamp().unix_micros(), 1_700_000_000_123_456);
}

#[test]
fn it_accepts_the_unix_epoch() {
    let updated_at = DnsResolutionUpdatedAt::from_unix_micros(0).expect("in range");

    assert_eq!(updated_at.value(), UNIX_EPOCH);
}

#[test]
fn it_accepts_the_last_microsecond_of_year_9999() {
    let updated_at = DnsResolutionUpdatedAt::from_unix_micros(MAX_UNIX_MICROS).expect("in range");

    assert_eq!(updated_at.unix_micros(), MAX_UNIX_MICROS);
}

#[test]
fn it_rejects_the_first_microsecond_of_year_10000() {
    let error =
        DnsResolutionUpdatedAt::from_unix_micros(MAX_UNIX_MICROS + 1).expect_err("out of range");

    assert!(error.message().contains("after 9999"));
}
