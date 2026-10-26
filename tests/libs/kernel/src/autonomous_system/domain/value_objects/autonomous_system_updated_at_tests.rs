use kernel::autonomous_system::domain::value_objects::autonomous_system_created_at::AutonomousSystemCreatedAt;
use kernel::autonomous_system::domain::value_objects::autonomous_system_updated_at::AutonomousSystemUpdatedAt;

#[test]
fn it_starts_at_the_creation_instant() {
    let created_at = AutonomousSystemCreatedAt::now();

    let updated_at = AutonomousSystemUpdatedAt::from_created_at(&created_at);

    assert_eq!(updated_at.unix_micros(), created_at.unix_micros());
}

#[test]
fn it_round_trips_through_unix_microseconds() {
    let updated_at = AutonomousSystemUpdatedAt::now();

    let restored = AutonomousSystemUpdatedAt::from_unix_micros(updated_at.unix_micros())
        .expect("a valid instant");

    assert_eq!(restored, updated_at);
}

#[test]
fn it_rejects_an_instant_after_year_9999() {
    assert!(AutonomousSystemUpdatedAt::from_unix_micros(u64::MAX).is_err());
}
