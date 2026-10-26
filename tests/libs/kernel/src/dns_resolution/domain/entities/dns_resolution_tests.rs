use kernel::dns_resolution::domain::entities::dns_resolution::DnsResolution;
use kernel::dns_resolution::domain::entities::dns_resolution_observation::DnsResolutionObservation;
use kernel::dns_resolution::domain::entities::dns_resolution_retraction::DnsResolutionRetraction;
use kernel::dns_resolution::domain::value_objects::dns_resolution_count::DnsResolutionCount;
use kernel::dns_resolution::domain::value_objects::dns_resolution_created_at::DnsResolutionCreatedAt;
use kernel::dns_resolution::domain::value_objects::dns_resolution_id::DnsResolutionId;
use kernel::dns_resolution::domain::value_objects::dns_resolution_source_id::DnsResolutionSourceId;
use kernel::dns_resolution::domain::value_objects::dns_resolution_updated_at::DnsResolutionUpdatedAt;
use kernel::dns_resolution::domain::value_objects::dns_resolution_window::DnsResolutionWindow;

use crate::src::dns_resolution::domain::entities::mothers::dns_resolution_mother::DnsResolutionMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_answer_mother::DnsResolutionAnswerMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_source_id_mother::DnsResolutionSourceIdMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_subject_mother::DnsResolutionSubjectMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_window_mother::DnsResolutionWindowMother;

const LOW: &str = "00000000-0000-4000-8000-000000000000";
const MIDDLE: &str = "77777777-7777-4777-8777-777777777777";
const HIGH: &str = "ffffffff-ffff-4fff-bfff-ffffffffffff";

const CREATED_AT: u64 = 1_700_000_000;
const NOW: u64 = 1_700_000_500;

fn source(value: &str) -> DnsResolutionSourceId {
    DnsResolutionSourceIdMother::create(value)
}

fn window(first_seen: u64, last_seen: u64) -> DnsResolutionWindow {
    DnsResolutionWindowMother::create(first_seen, last_seen)
}

fn count(value: u64) -> DnsResolutionCount {
    DnsResolutionCount::from_u64(value).expect("a valid count")
}

fn created_at(seconds: u64) -> DnsResolutionCreatedAt {
    DnsResolutionCreatedAt::from_unix_micros(seconds * 1_000_000).expect("in range")
}

fn now() -> DnsResolutionUpdatedAt {
    DnsResolutionUpdatedAt::from_unix_micros(NOW * 1_000_000).expect("in range")
}

/// The example resolution, first reported by `source_id` over `window` at
/// [`CREATED_AT`].
fn recorded(
    source_id: &str,
    window: DnsResolutionWindow,
    count: Option<DnsResolutionCount>,
) -> DnsResolution {
    DnsResolution::record(
        DnsResolutionSubjectMother::create("example.com"),
        DnsResolutionAnswerMother::ipv4("93.184.216.34"),
        source(source_id),
        window,
        count,
        created_at(CREATED_AT),
    )
}

fn observed(
    resolution: &DnsResolution,
    source_id: &str,
    window: DnsResolutionWindow,
    count: Option<DnsResolutionCount>,
) -> (DnsResolution, DnsResolutionObservation) {
    resolution
        .observe(source(source_id), window, count, now())
        .expect("the report must change the resolution")
}

fn sources(resolution: &DnsResolution) -> Vec<String> {
    resolution
        .observations()
        .iter()
        .map(|observation| observation.source_id().to_string())
        .collect()
}

#[test]
fn it_derives_its_id_from_its_endpoints() {
    let subject = DnsResolutionSubjectMother::random();
    let answer = DnsResolutionAnswerMother::random();

    let resolution = DnsResolutionMother::create(subject.clone(), answer);

    assert_eq!(
        *resolution.id(),
        DnsResolutionId::from_endpoints(&subject, &answer)
    );
}

#[test]
fn it_matches_the_documented_vector() {
    assert_eq!(
        DnsResolutionMother::example().id().to_string(),
        "2da44331-2011-5539-b65e-9ff44db9c139"
    );
}

#[test]
fn it_derives_the_same_id_whichever_source_reports_it_first() {
    let one = recorded(LOW, window(100, 200), None);
    let other = recorded(HIGH, window(500, 600), Some(count(9)));

    assert_eq!(one.id(), other.id());
}

#[test]
fn it_keeps_its_endpoints_and_creation_instant() {
    let resolution = recorded(LOW, window(100, 200), None);

    assert_eq!(
        *resolution.subject(),
        DnsResolutionSubjectMother::create("example.com")
    );
    assert_eq!(
        *resolution.answer(),
        DnsResolutionAnswerMother::ipv4("93.184.216.34")
    );
    assert_eq!(*resolution.created_at(), created_at(CREATED_AT));
}

#[test]
fn it_holds_one_observation_with_a_count_of_one_when_none_is_reported() {
    let resolution = recorded(LOW, window(100, 200), None);

    let [observation] = resolution.observations() else {
        panic!("a recorded resolution holds exactly one observation");
    };
    assert_eq!(*observation.source_id(), source(LOW));
    assert_eq!(*observation.window(), window(100, 200));
    assert_eq!(observation.count(), DnsResolutionCount::one());
    assert_eq!(*observation.created_at(), created_at(CREATED_AT));
    assert_eq!(
        observation.updated_at().unix_micros(),
        created_at(CREATED_AT).unix_micros()
    );
}

#[test]
fn it_keeps_the_count_its_first_source_reports() {
    let resolution = recorded(LOW, window(100, 200), Some(count(42)));

    assert_eq!(resolution.observations()[0].count(), count(42));
}

#[test]
fn it_finds_the_observation_of_a_source() {
    let resolution = recorded(LOW, window(100, 200), None);

    assert_eq!(
        resolution.observation(&source(LOW)),
        Some(&resolution.observations()[0])
    );
    assert_eq!(resolution.observation(&source(HIGH)), None);
}

#[test]
fn it_adds_the_observation_of_a_new_source() {
    let original = recorded(LOW, window(100, 200), None);

    let (resolution, observation) = observed(&original, HIGH, window(500, 600), None);

    assert_eq!(resolution.observations().len(), 2);
    assert_eq!(*observation.source_id(), source(HIGH));
    assert_eq!(*observation.window(), window(500, 600));
    assert_eq!(observation.count(), DnsResolutionCount::one());
    assert_eq!(resolution.observation(&source(HIGH)), Some(&observation));
    assert_eq!(
        resolution.observation(&source(LOW)),
        original.observation(&source(LOW))
    );
}

#[test]
fn it_creates_the_observation_of_a_new_source_now() {
    let original = recorded(LOW, window(100, 200), None);

    let (_, observation) = observed(&original, HIGH, window(500, 600), None);

    assert_eq!(observation.created_at().unix_micros(), now().unix_micros());
    assert_eq!(*observation.updated_at(), now());
}

#[test]
fn it_keeps_the_count_a_new_source_reports() {
    let original = recorded(LOW, window(100, 200), None);

    let (_, observation) = observed(&original, HIGH, window(500, 600), Some(count(9)));

    assert_eq!(observation.count(), count(9));
}

#[test]
fn it_orders_its_observations_by_source_id() {
    let original = recorded(HIGH, window(100, 200), None);

    let (with_low, _) = observed(&original, LOW, window(100, 200), None);
    let (with_middle, _) = observed(&with_low, MIDDLE, window(100, 200), None);

    assert_eq!(sources(&with_low), vec![LOW, HIGH]);
    assert_eq!(sources(&with_middle), vec![LOW, MIDDLE, HIGH]);
}

#[test]
fn it_orders_the_observations_of_random_sources_by_source_id() {
    let mut resolution = DnsResolutionMother::random();
    for _ in 0..8 {
        resolution = resolution
            .observe(
                DnsResolutionSourceIdMother::random(),
                DnsResolutionWindowMother::random(),
                None,
                now(),
            )
            .expect("a new source is a change")
            .0;
    }

    let ids: Vec<_> = resolution
        .observations()
        .iter()
        .map(|observation| *observation.source_id().value())
        .collect();
    let mut sorted = ids.clone();
    sorted.sort();
    assert_eq!(ids.len(), 9);
    assert_eq!(ids, sorted);
}

#[test]
fn it_keeps_its_identity_when_observed() {
    let original = recorded(LOW, window(100, 200), None);

    let (resolution, _) = observed(&original, HIGH, window(500, 600), None);

    assert_eq!(resolution.id(), original.id());
    assert_eq!(resolution.subject(), original.subject());
    assert_eq!(resolution.answer(), original.answer());
    assert_eq!(resolution.created_at(), original.created_at());
}

#[test]
fn it_does_not_change_when_observed() {
    let original = recorded(LOW, window(100, 200), None);
    let copy = original.clone();

    let _ = original.observe(source(HIGH), window(500, 600), None, now());
    let _ = original.observe(source(LOW), window(50, 300), Some(count(9)), now());

    assert_eq!(original, copy);
}

#[test]
fn it_widens_the_window_of_a_known_source() {
    let original = recorded(LOW, window(200, 300), None);

    let (earlier, observation) = observed(&original, LOW, window(100, 250), None);
    let (later, _) = observed(&earlier, LOW, window(250, 400), None);

    assert_eq!(earlier.observations().len(), 1);
    assert_eq!(*observation.window(), window(100, 300));
    assert_eq!(*later.observations()[0].window(), window(100, 400));
}

#[test]
fn it_keeps_the_creation_of_a_known_source_and_records_the_change() {
    let original = recorded(LOW, window(200, 300), None);

    let (_, observation) = observed(&original, LOW, window(100, 300), None);

    assert_eq!(*observation.created_at(), created_at(CREATED_AT));
    assert_eq!(*observation.updated_at(), now());
}

#[test]
fn it_keeps_the_larger_count_of_a_known_source() {
    let original = recorded(LOW, window(200, 300), Some(count(5)));

    let (_, larger) = observed(&original, LOW, window(200, 300), Some(count(8)));
    let (_, wider) = observed(&original, LOW, window(100, 400), Some(count(3)));

    assert_eq!(larger.count(), count(8));
    assert_eq!(*larger.window(), window(200, 300));
    assert_eq!(wider.count(), count(5));
    assert_eq!(*wider.window(), window(100, 400));
}

#[test]
fn it_ignores_a_report_inside_the_window_with_a_smaller_count() {
    let original = recorded(LOW, window(200, 300), Some(count(5)));

    assert_eq!(
        original.observe(source(LOW), window(220, 280), Some(count(3)), now()),
        None
    );
}

#[test]
fn it_ignores_a_report_inside_the_window_with_the_same_count() {
    let original = recorded(LOW, window(200, 300), Some(count(5)));

    assert_eq!(
        original.observe(source(LOW), window(200, 300), Some(count(5)), now()),
        None
    );
}

#[test]
fn it_ignores_a_report_inside_the_window_without_a_count() {
    let original = recorded(LOW, window(200, 300), Some(count(5)));

    assert_eq!(
        original.observe(source(LOW), window(220, 280), None, now()),
        None
    );
}

#[test]
fn it_ignores_the_same_report_twice() {
    let original = recorded(LOW, window(200, 300), None);

    let (resolution, _) = observed(&original, HIGH, window(500, 600), Some(count(4)));

    assert_eq!(
        resolution.observe(source(HIGH), window(500, 600), Some(count(4)), now()),
        None
    );
}

#[test]
fn it_reports_that_an_unknown_source_did_not_observe_it() {
    let resolution = recorded(LOW, window(100, 200), None);

    assert_eq!(
        resolution.retract(&source(HIGH)),
        DnsResolutionRetraction::NotObserved
    );
}

#[test]
fn it_reports_the_retraction_of_its_last_observation() {
    let resolution = recorded(LOW, window(100, 200), None);

    assert_eq!(
        resolution.retract(&source(LOW)),
        DnsResolutionRetraction::LastObservationRetracted
    );
}

#[test]
fn it_keeps_the_other_observations_when_one_is_retracted() {
    let original = recorded(LOW, window(100, 200), None);
    let (with_high, _) = observed(&original, HIGH, window(500, 600), None);
    let (resolution, _) = observed(&with_high, MIDDLE, window(300, 400), None);

    let DnsResolutionRetraction::Retracted(retracted) = resolution.retract(&source(MIDDLE)) else {
        panic!("two sources still assert the resolution");
    };

    assert_eq!(sources(&retracted), vec![LOW, HIGH]);
    assert_eq!(retracted.observation(&source(MIDDLE)), None);
    assert_eq!(
        retracted.observation(&source(HIGH)),
        resolution.observation(&source(HIGH))
    );
    assert_eq!(retracted.id(), resolution.id());
    assert_eq!(retracted.subject(), resolution.subject());
    assert_eq!(retracted.answer(), resolution.answer());
    assert_eq!(retracted.created_at(), resolution.created_at());
}

#[test]
fn it_reports_the_last_retraction_after_the_others() {
    let original = recorded(LOW, window(100, 200), None);
    let (resolution, _) = observed(&original, HIGH, window(500, 600), None);

    let DnsResolutionRetraction::Retracted(retracted) = resolution.retract(&source(LOW)) else {
        panic!("the other source still asserts the resolution");
    };

    assert_eq!(
        retracted.retract(&source(LOW)),
        DnsResolutionRetraction::NotObserved
    );
    assert_eq!(
        retracted.retract(&source(HIGH)),
        DnsResolutionRetraction::LastObservationRetracted
    );
}

#[test]
fn it_does_not_change_when_retracted() {
    let original = recorded(LOW, window(100, 200), None);
    let (resolution, _) = observed(&original, HIGH, window(500, 600), None);
    let copy = resolution.clone();

    let _ = resolution.retract(&source(LOW));

    assert_eq!(resolution, copy);
}

#[test]
fn it_is_restored_with_every_field_and_its_derived_id() {
    let subject = DnsResolutionSubjectMother::random();
    let answer = DnsResolutionAnswerMother::random();
    let observation = DnsResolutionObservation::new(
        source(LOW),
        window(100, 200),
        Some(count(3)),
        created_at(CREATED_AT),
    );

    let restored = DnsResolution::restore(
        subject.clone(),
        answer,
        created_at(CREATED_AT),
        vec![observation.clone()],
    )
    .expect("one observation is a valid resolution");

    assert_eq!(
        *restored.id(),
        DnsResolutionId::from_endpoints(&subject, &answer)
    );
    assert_eq!(*restored.subject(), subject);
    assert_eq!(*restored.answer(), answer);
    assert_eq!(*restored.created_at(), created_at(CREATED_AT));
    assert_eq!(restored.observations(), &[observation]);
}

#[test]
fn it_is_restored_equal_to_the_resolution_it_was_stored_from() {
    let original = recorded(LOW, window(100, 200), None);
    let (resolution, _) = observed(&original, HIGH, window(500, 600), Some(count(2)));

    let restored = DnsResolution::restore(
        resolution.subject().clone(),
        *resolution.answer(),
        *resolution.created_at(),
        resolution.observations().to_vec(),
    );

    assert_eq!(restored, Some(resolution));
}

#[test]
fn it_orders_restored_observations_by_source_id() {
    let observation = |source_id: &str| {
        DnsResolutionObservation::new(
            source(source_id),
            window(100, 200),
            None,
            created_at(CREATED_AT),
        )
    };

    let restored = DnsResolution::restore(
        DnsResolutionSubjectMother::random(),
        DnsResolutionAnswerMother::random(),
        created_at(CREATED_AT),
        vec![observation(HIGH), observation(LOW), observation(MIDDLE)],
    )
    .expect("three sources are a valid resolution");

    assert_eq!(sources(&restored), vec![LOW, MIDDLE, HIGH]);
}

#[test]
fn it_is_not_restored_without_observations() {
    assert_eq!(
        DnsResolution::restore(
            DnsResolutionSubjectMother::random(),
            DnsResolutionAnswerMother::random(),
            created_at(CREATED_AT),
            Vec::new(),
        ),
        None
    );
}

#[test]
fn it_is_not_restored_with_a_source_twice() {
    let observation = |source_id: &str, window: DnsResolutionWindow| {
        DnsResolutionObservation::new(source(source_id), window, None, created_at(CREATED_AT))
    };

    for observations in [
        vec![
            observation(LOW, window(100, 200)),
            observation(LOW, window(300, 400)),
        ],
        vec![
            observation(LOW, window(100, 200)),
            observation(HIGH, window(100, 200)),
            observation(LOW, window(100, 200)),
        ],
    ] {
        assert_eq!(
            DnsResolution::restore(
                DnsResolutionSubjectMother::random(),
                DnsResolutionAnswerMother::random(),
                created_at(CREATED_AT),
                observations,
            ),
            None
        );
    }
}

#[test]
fn it_spans_the_window_of_its_only_observation() {
    assert_eq!(
        recorded(LOW, window(100, 200), None).window(),
        window(100, 200)
    );
}

#[test]
fn it_spans_the_windows_of_every_observation() {
    let original = recorded(MIDDLE, window(150, 400), None);
    let (with_high, _) = observed(&original, HIGH, window(100, 200), None);
    let (resolution, _) = observed(&with_high, LOW, window(50, 60), None);

    assert_eq!(resolution.window(), window(50, 400));
}

#[test]
fn it_spans_disjoint_observations_across_the_gap() {
    let original = recorded(LOW, window(100, 200), None);

    let (resolution, _) = observed(&original, HIGH, window(500, 600), None);

    assert_eq!(resolution.window(), window(100, 600));
}

#[test]
fn it_narrows_its_window_when_an_observation_is_retracted() {
    let original = recorded(LOW, window(100, 200), None);
    let (resolution, _) = observed(&original, HIGH, window(500, 600), None);

    let DnsResolutionRetraction::Retracted(retracted) = resolution.retract(&source(HIGH)) else {
        panic!("the other source still asserts the resolution");
    };

    assert_eq!(retracted.window(), window(100, 200));
}
