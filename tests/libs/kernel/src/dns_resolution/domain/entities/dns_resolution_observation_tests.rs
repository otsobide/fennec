use kernel::dns_resolution::domain::entities::dns_resolution_observation::DnsResolutionObservation;
use kernel::dns_resolution::domain::value_objects::dns_resolution_count::DnsResolutionCount;
use kernel::dns_resolution::domain::value_objects::dns_resolution_created_at::DnsResolutionCreatedAt;
use kernel::dns_resolution::domain::value_objects::dns_resolution_updated_at::DnsResolutionUpdatedAt;

use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_source_id_mother::DnsResolutionSourceIdMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_window_mother::DnsResolutionWindowMother;

const CREATED_AT: u64 = 1_700_000_000;
const LATER: u64 = 1_700_000_500;

fn count(value: u64) -> DnsResolutionCount {
    DnsResolutionCount::from_u64(value).expect("a valid count")
}

fn created_at(seconds: u64) -> DnsResolutionCreatedAt {
    DnsResolutionCreatedAt::from_unix_micros(seconds * 1_000_000).expect("in range")
}

fn updated_at(seconds: u64) -> DnsResolutionUpdatedAt {
    DnsResolutionUpdatedAt::from_unix_micros(seconds * 1_000_000).expect("in range")
}

/// An observation of a random source over `[200, 300]`, created at
/// [`CREATED_AT`].
fn observation(count: Option<DnsResolutionCount>) -> DnsResolutionObservation {
    DnsResolutionObservation::new(
        DnsResolutionSourceIdMother::random(),
        DnsResolutionWindowMother::create(200, 300),
        count,
        created_at(CREATED_AT),
    )
}

#[test]
fn it_keeps_its_source_and_window() {
    let source_id = DnsResolutionSourceIdMother::random();
    let window = DnsResolutionWindowMother::random();

    let observation =
        DnsResolutionObservation::new(source_id.clone(), window, None, created_at(CREATED_AT));

    assert_eq!(*observation.source_id(), source_id);
    assert_eq!(*observation.window(), window);
}

#[test]
fn it_counts_one_sighting_when_no_count_is_reported() {
    assert_eq!(observation(None).count(), DnsResolutionCount::one());
}

#[test]
fn it_keeps_the_reported_count() {
    assert_eq!(observation(Some(count(42))).count(), count(42));
}

#[test]
fn it_is_last_changed_when_it_is_created() {
    let observation = observation(None);

    assert_eq!(*observation.created_at(), created_at(CREATED_AT));
    assert_eq!(
        observation.updated_at().unix_micros(),
        observation.created_at().unix_micros()
    );
}

#[test]
fn it_is_restored_with_every_field() {
    let source_id = DnsResolutionSourceIdMother::random();
    let window = DnsResolutionWindowMother::random();

    let restored = DnsResolutionObservation::restore(
        source_id.clone(),
        window,
        count(7),
        created_at(CREATED_AT),
        updated_at(LATER),
    );

    assert_eq!(*restored.source_id(), source_id);
    assert_eq!(*restored.window(), window);
    assert_eq!(restored.count(), count(7));
    assert_eq!(*restored.created_at(), created_at(CREATED_AT));
    assert_eq!(*restored.updated_at(), updated_at(LATER));
}

#[test]
fn it_widens_its_window_to_an_earlier_first_seen() {
    let original = observation(Some(count(5)));

    let merged = original
        .merged(
            DnsResolutionWindowMother::create(100, 250),
            None,
            updated_at(LATER),
        )
        .expect("an earlier first_seen widens the window");

    assert_eq!(
        *merged.window(),
        DnsResolutionWindowMother::create(100, 300)
    );
    assert_eq!(merged.count(), count(5));
}

#[test]
fn it_widens_its_window_to_a_later_last_seen() {
    let merged = observation(None)
        .merged(
            DnsResolutionWindowMother::create(250, 400),
            None,
            updated_at(LATER),
        )
        .expect("a later last_seen widens the window");

    assert_eq!(
        *merged.window(),
        DnsResolutionWindowMother::create(200, 400)
    );
}

#[test]
fn it_widens_its_window_across_a_gap() {
    let merged = observation(None)
        .merged(
            DnsResolutionWindowMother::create(900, 1_000),
            None,
            updated_at(LATER),
        )
        .expect("a disjoint window widens the window");

    assert_eq!(
        *merged.window(),
        DnsResolutionWindowMother::create(200, 1_000)
    );
}

#[test]
fn it_takes_a_larger_count() {
    let original = observation(Some(count(5)));

    let merged = original
        .merged(*original.window(), Some(count(8)), updated_at(LATER))
        .expect("a larger count is a change");

    assert_eq!(merged.count(), count(8));
    assert_eq!(merged.window(), original.window());
}

#[test]
fn it_takes_a_count_of_two_over_an_unreported_count() {
    let original = observation(None);

    let merged = original
        .merged(*original.window(), Some(count(2)), updated_at(LATER))
        .expect("two is more than the one sighting assumed");

    assert_eq!(merged.count(), count(2));
}

#[test]
fn it_keeps_its_count_when_a_wider_report_has_a_smaller_count() {
    let merged = observation(Some(count(5)))
        .merged(
            DnsResolutionWindowMother::create(100, 400),
            Some(count(3)),
            updated_at(LATER),
        )
        .expect("the window widens");

    assert_eq!(
        *merged.window(),
        DnsResolutionWindowMother::create(100, 400)
    );
    assert_eq!(merged.count(), count(5));
}

#[test]
fn it_records_when_it_changed_and_keeps_its_source_and_creation() {
    let original = observation(None);

    let merged = original
        .merged(
            DnsResolutionWindowMother::create(100, 400),
            None,
            updated_at(LATER),
        )
        .expect("the window widens");

    assert_eq!(merged.source_id(), original.source_id());
    assert_eq!(merged.created_at(), original.created_at());
    assert_eq!(*merged.updated_at(), updated_at(LATER));
}

#[test]
fn it_ignores_a_report_inside_its_window_with_a_smaller_count() {
    let original = observation(Some(count(5)));

    assert_eq!(
        original.merged(
            DnsResolutionWindowMother::create(220, 280),
            Some(count(3)),
            updated_at(LATER)
        ),
        None
    );
}

#[test]
fn it_ignores_a_report_inside_its_window_without_a_count() {
    let original = observation(Some(count(5)));

    assert_eq!(
        original.merged(
            DnsResolutionWindowMother::create(220, 280),
            None,
            updated_at(LATER)
        ),
        None
    );
}

#[test]
fn it_ignores_a_report_on_the_edges_of_its_window_with_the_same_count() {
    let original = observation(Some(count(5)));

    assert_eq!(
        original.merged(*original.window(), Some(count(5)), updated_at(LATER)),
        None
    );
}

#[test]
fn it_ignores_a_count_of_one_over_an_unreported_count() {
    let original = observation(None);

    assert_eq!(
        original.merged(*original.window(), Some(count(1)), updated_at(LATER)),
        None
    );
}

#[test]
fn it_ignores_the_same_report_twice() {
    let wider = DnsResolutionWindowMother::create(100, 400);
    let merged = observation(Some(count(5)))
        .merged(wider, Some(count(9)), updated_at(LATER))
        .expect("the first report is a change");

    assert_eq!(
        merged.merged(wider, Some(count(9)), updated_at(LATER + 1)),
        None
    );
}

#[test]
fn it_does_not_change_when_merged() {
    let original = observation(Some(count(5)));
    let copy = original.clone();

    let _ = original.merged(
        DnsResolutionWindowMother::create(100, 400),
        Some(count(9)),
        updated_at(LATER),
    );

    assert_eq!(original, copy);
}
