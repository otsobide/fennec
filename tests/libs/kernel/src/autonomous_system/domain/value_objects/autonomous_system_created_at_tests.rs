use std::time::UNIX_EPOCH;

use kernel::autonomous_system::domain::value_objects::autonomous_system_created_at::AutonomousSystemCreatedAt;

#[test]
fn it_records_the_current_instant_with_microsecond_precision() {
    let created_at = AutonomousSystemCreatedAt::now();

    let nanos = created_at
        .value()
        .duration_since(UNIX_EPOCH)
        .expect("after the epoch")
        .subsec_nanos();
    assert_eq!(nanos % 1_000, 0);
}

#[test]
fn it_round_trips_through_unix_microseconds() {
    let created_at = AutonomousSystemCreatedAt::now();

    let reloaded = AutonomousSystemCreatedAt::from_unix_micros(created_at.unix_micros())
        .expect("valid instant");

    assert_eq!(reloaded, created_at);
}

#[test]
fn it_rejects_instants_after_year_9999() {
    let error = AutonomousSystemCreatedAt::from_unix_micros(u64::MAX).expect_err("out of range");

    assert!(error.message().contains("after 9999"));
}
