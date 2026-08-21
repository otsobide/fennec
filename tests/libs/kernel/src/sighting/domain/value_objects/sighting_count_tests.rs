use kernel::sighting::domain::value_objects::sighting_count::SightingCount;

#[test]
fn it_starts_at_one() {
    assert_eq!(SightingCount::one().value(), 1);
}

#[test]
fn it_accepts_the_smallest_valid_count() {
    let count = SightingCount::from_u64(1).expect("one observation is valid");

    assert_eq!(count.value(), 1);
}

#[test]
fn it_rejects_a_zero_count() {
    let error = SightingCount::from_u64(0).expect_err("zero observations must be rejected");

    assert!(error.message().contains("at least 1"));
}

#[test]
fn it_increments_by_one() {
    let count = SightingCount::one().incremented();

    assert_eq!(count.value(), 2);
}
