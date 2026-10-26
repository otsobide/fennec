use std::time::{Duration, UNIX_EPOCH};

use kernel::dns_resolution::domain::value_objects::dns_resolution_window::DnsResolutionWindow;
use shared_valueobject::domain::timestamps::microsecond_timestamp::MAX_UNIX_MICROS;

use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_window_mother::DnsResolutionWindowMother;

/// 9999-12-31T23:59:59Z, the last whole second before year 10000.
const LAST_SECOND_OF_9999: u64 = 253_402_300_799;

fn window(first_seen: u64, last_seen: u64) -> DnsResolutionWindow {
    DnsResolutionWindowMother::create(first_seen, last_seen)
}

#[test]
fn it_keeps_both_instants() {
    let window = window(1_600_000_000, 1_700_000_000);

    assert_eq!(window.first_seen_micros(), 1_600_000_000_000_000);
    assert_eq!(window.last_seen_micros(), 1_700_000_000_000_000);
    assert_eq!(
        window.first_seen(),
        UNIX_EPOCH + Duration::from_secs(1_600_000_000)
    );
    assert_eq!(
        window.last_seen(),
        UNIX_EPOCH + Duration::from_secs(1_700_000_000)
    );
}

#[test]
fn it_takes_first_seen_as_last_seen_when_last_seen_is_missing() {
    let window =
        DnsResolutionWindow::from_unix_seconds(1_600_000_000, None).expect("a valid window");

    assert_eq!(window.first_seen_micros(), 1_600_000_000_000_000);
    assert_eq!(window.last_seen_micros(), 1_600_000_000_000_000);
    assert_eq!(window, self::window(1_600_000_000, 1_600_000_000));
}

#[test]
fn it_accepts_a_single_instant() {
    let window = window(1_600_000_000, 1_600_000_000);

    assert_eq!(window.first_seen(), window.last_seen());
}

#[test]
fn it_rejects_last_seen_one_second_before_first_seen() {
    let error = DnsResolutionWindow::from_unix_seconds(1_600_000_000, Some(1_599_999_999))
        .expect_err("last_seen must not be before first_seen");

    assert_eq!(
        error.message(),
        "dns resolution last_seen (1599999999) must not be before first_seen (1600000000)"
    );
}

#[test]
fn it_accepts_the_unix_epoch() {
    let window = window(0, 0);

    assert_eq!(window.first_seen(), UNIX_EPOCH);
    assert_eq!(window.last_seen(), UNIX_EPOCH);
}

#[test]
fn it_accepts_the_last_second_of_year_9999() {
    let window = window(LAST_SECOND_OF_9999, LAST_SECOND_OF_9999);

    assert_eq!(window.last_seen_micros(), LAST_SECOND_OF_9999 * 1_000_000);
}

#[test]
fn it_rejects_a_first_seen_in_year_10000() {
    let error = DnsResolutionWindow::from_unix_seconds(LAST_SECOND_OF_9999 + 1, None)
        .expect_err("year 10000 is out of range");

    assert_eq!(
        error.message(),
        "dns resolution first_seen must be a unix time in seconds before year 10000, got 253402300800"
    );
}

#[test]
fn it_rejects_a_last_seen_in_year_10000() {
    let error = DnsResolutionWindow::from_unix_seconds(0, Some(LAST_SECOND_OF_9999 + 1))
        .expect_err("year 10000 is out of range");

    assert_eq!(
        error.message(),
        "dns resolution last_seen must be a unix time in seconds before year 10000, got 253402300800"
    );
}

#[test]
fn it_rejects_seconds_that_overflow_microseconds() {
    let error = DnsResolutionWindow::from_unix_seconds(u64::MAX, None)
        .expect_err("u64::MAX seconds do not fit in microseconds");

    assert_eq!(
        error.message(),
        format!(
            "dns resolution first_seen must be a unix time in seconds before year 10000, got {}",
            u64::MAX
        )
    );
}

#[test]
fn it_round_trips_through_unix_microseconds() {
    let original = DnsResolutionWindowMother::random();

    let reloaded = DnsResolutionWindow::from_unix_micros(
        original.first_seen_micros(),
        original.last_seen_micros(),
    )
    .expect("a valid window");

    assert_eq!(reloaded, original);
}

#[test]
fn it_keeps_microseconds_read_back_from_a_store() {
    let window =
        DnsResolutionWindow::from_unix_micros(1_600_000_000_123_456, 1_600_000_000_123_457)
            .expect("a valid window");

    assert_eq!(window.first_seen_micros(), 1_600_000_000_123_456);
    assert_eq!(window.last_seen_micros(), 1_600_000_000_123_457);
}

#[test]
fn it_rejects_a_last_seen_one_microsecond_before_first_seen_from_a_store() {
    let error = DnsResolutionWindow::from_unix_micros(1_600_000_000_000_001, 1_600_000_000_000_000)
        .expect_err("last_seen must not be before first_seen");

    assert!(error.message().contains("must not be before first_seen"));
}

#[test]
fn it_accepts_the_latest_instant_from_a_store() {
    let window =
        DnsResolutionWindow::from_unix_micros(MAX_UNIX_MICROS, MAX_UNIX_MICROS).expect("in range");

    assert_eq!(window.last_seen_micros(), MAX_UNIX_MICROS);
}

#[test]
fn it_rejects_an_instant_after_year_9999_from_a_store() {
    assert!(
        DnsResolutionWindow::from_unix_micros(0, MAX_UNIX_MICROS + 1)
            .expect_err("out of range")
            .message()
            .contains("after 9999")
    );
    assert!(
        DnsResolutionWindow::from_unix_micros(MAX_UNIX_MICROS + 1, MAX_UNIX_MICROS + 1)
            .expect_err("out of range")
            .message()
            .contains("after 9999")
    );
}

#[test]
fn it_merges_into_the_window_that_covers_both() {
    let merged = window(100, 200).merged(&window(150, 400));

    assert_eq!(merged, window(100, 400));
}

#[test]
fn it_merges_disjoint_windows_across_the_gap() {
    let merged = window(100, 200).merged(&window(500, 600));

    assert_eq!(merged, window(100, 600));
}

#[test]
fn it_merges_into_the_outer_window_when_one_contains_the_other() {
    let outer = window(100, 600);

    assert_eq!(outer.merged(&window(200, 300)), outer);
    assert_eq!(window(200, 300).merged(&outer), outer);
}

#[test]
fn it_merges_commutatively() {
    let one = DnsResolutionWindowMother::random();
    let other = DnsResolutionWindowMother::random();

    assert_eq!(one.merged(&other), other.merged(&one));
    assert_eq!(
        window(100, 200).merged(&window(500, 600)),
        window(500, 600).merged(&window(100, 200))
    );
}

#[test]
fn it_merges_idempotently() {
    let one = DnsResolutionWindowMother::random();
    let other = DnsResolutionWindowMother::random();

    assert_eq!(one.merged(&one), one);
    assert_eq!(one.merged(&other).merged(&other), one.merged(&other));
}

#[test]
fn it_merges_associatively() {
    let (a, b, c) = (window(300, 400), window(100, 150), window(500, 900));

    assert_eq!(a.merged(&b).merged(&c), a.merged(&b.merged(&c)));
}

#[test]
fn it_overlaps_a_window_that_starts_where_it_ends() {
    let earlier = window(100, 200);
    let later = window(200, 300);

    assert!(earlier.overlaps(&later));
    assert!(later.overlaps(&earlier));
}

#[test]
fn it_does_not_overlap_a_window_that_starts_one_second_after_it_ends() {
    let earlier = window(100, 200);
    let later = window(201, 300);

    assert!(!earlier.overlaps(&later));
    assert!(!later.overlaps(&earlier));
}

#[test]
fn it_does_not_overlap_a_window_that_starts_one_microsecond_after_it_ends() {
    let earlier = DnsResolutionWindow::from_unix_micros(100, 200).expect("a valid window");
    let later = DnsResolutionWindow::from_unix_micros(201, 300).expect("a valid window");

    assert!(!earlier.overlaps(&later));
    assert!(!later.overlaps(&earlier));
}

#[test]
fn it_overlaps_a_window_it_contains() {
    let outer = window(100, 600);
    let inner = window(200, 300);

    assert!(outer.overlaps(&inner));
    assert!(inner.overlaps(&outer));
}

#[test]
fn it_overlaps_itself() {
    let window = DnsResolutionWindowMother::random();

    assert!(window.overlaps(&window));
    assert!(self::window(100, 100).overlaps(&self::window(100, 100)));
}

#[test]
fn it_overlaps_a_single_instant_on_its_edges() {
    let window = window(100, 200);

    assert!(window.overlaps(&self::window(100, 100)));
    assert!(window.overlaps(&self::window(200, 200)));
    assert!(!window.overlaps(&self::window(99, 99)));
    assert!(!window.overlaps(&self::window(201, 201)));
}
