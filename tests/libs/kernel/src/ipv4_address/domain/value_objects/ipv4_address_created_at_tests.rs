use std::time::UNIX_EPOCH;

use kernel::ipv4_address::domain::value_objects::ipv4_address_created_at::Ipv4AddressCreatedAt;

#[test]
fn it_records_the_current_instant_with_microsecond_precision() {
    let created_at = Ipv4AddressCreatedAt::now();

    let nanos = created_at
        .value()
        .duration_since(UNIX_EPOCH)
        .expect("after the epoch")
        .subsec_nanos();
    assert_eq!(nanos % 1_000, 0);
}

#[test]
fn it_round_trips_through_unix_microseconds() {
    let created_at = Ipv4AddressCreatedAt::now();

    let reloaded =
        Ipv4AddressCreatedAt::from_unix_micros(created_at.unix_micros()).expect("valid instant");

    assert_eq!(reloaded, created_at);
}

#[test]
fn it_rejects_instants_after_year_9999() {
    let error = Ipv4AddressCreatedAt::from_unix_micros(u64::MAX).expect_err("out of range");

    assert!(error.message().contains("after 9999"));
}
