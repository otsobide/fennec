use kernel::dns_resolution::domain::value_objects::dns_resolution_count::DnsResolutionCount;

fn count(value: u64) -> DnsResolutionCount {
    DnsResolutionCount::from_u64(value).expect("a valid count")
}

#[test]
fn it_starts_at_one() {
    assert_eq!(DnsResolutionCount::one().value(), 1);
    assert_eq!(DnsResolutionCount::one(), count(1));
}

#[test]
fn it_accepts_the_smallest_count() {
    assert_eq!(count(1).value(), 1);
}

#[test]
fn it_rejects_a_zero_count() {
    let error = DnsResolutionCount::from_u64(0).expect_err("zero sightings must be rejected");

    assert_eq!(
        error.message(),
        "dns resolution count must be from 1 to 9223372036854775807, got 0"
    );
}

#[test]
fn it_accepts_the_largest_count_a_store_can_hold() {
    assert_eq!(count(i64::MAX as u64).value(), 9_223_372_036_854_775_807);
}

#[test]
fn it_rejects_a_count_one_over_what_a_store_can_hold() {
    let error = DnsResolutionCount::from_u64(i64::MAX as u64 + 1)
        .expect_err("a count above i64::MAX must be rejected");

    assert_eq!(
        error.message(),
        "dns resolution count must be from 1 to 9223372036854775807, got 9223372036854775808"
    );
}

#[test]
fn it_rejects_the_largest_unsigned_count() {
    assert!(DnsResolutionCount::from_u64(u64::MAX).is_err());
}

#[test]
fn it_keeps_the_larger_count_in_either_order() {
    assert_eq!(count(3).max(count(8)), count(8));
    assert_eq!(count(8).max(count(3)), count(8));
}

#[test]
fn it_keeps_an_equal_count() {
    assert_eq!(count(5).max(count(5)), count(5));
}

#[test]
fn it_orders_by_value() {
    assert!(count(1) < count(2));
    assert!(count(i64::MAX as u64) > count(1));
}
