use std::time::{Duration, UNIX_EPOCH};

use kernel::dns_resolution::domain::entities::dns_resolution::DnsResolution;
use kernel::dns_resolution::domain::events::create_dns_resolution_observed_event::create_dns_resolution_observed_event;
use kernel::dns_resolution::domain::events::dns_resolution_observed_event::DnsResolutionObservedEvent;
use kernel::dns_resolution::domain::value_objects::dns_resolution_count::DnsResolutionCount;
use kernel::dns_resolution::domain::value_objects::dns_resolution_created_at::DnsResolutionCreatedAt;
use kernel::dns_resolution::domain::value_objects::dns_resolution_updated_at::DnsResolutionUpdatedAt;
use shared_domain_events::domain::domain_event::DomainEvent;
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::src::dns_resolution::domain::entities::mothers::dns_resolution_mother::DnsResolutionMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_answer_mother::DnsResolutionAnswerMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_source_id_mother::DnsResolutionSourceIdMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_subject_mother::DnsResolutionSubjectMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_window_mother::DnsResolutionWindowMother;

const SOURCE: &str = "550e8400-e29b-41d4-a716-446655440000";

fn seconds(value: u64) -> PrimitiveValue {
    PrimitiveValue::Timestamp(UNIX_EPOCH + Duration::from_secs(value))
}

fn count(value: u64) -> DnsResolutionCount {
    DnsResolutionCount::from_u64(value).expect("a valid count")
}

/// `example.com` to `93.184.216.34`, first reported by `SOURCE` in 2019 with
/// a count of 20.
fn reported_in_2019() -> DnsResolution {
    DnsResolution::record(
        DnsResolutionSubjectMother::create("example.com"),
        DnsResolutionAnswerMother::ipv4("93.184.216.34"),
        DnsResolutionSourceIdMother::create(SOURCE),
        DnsResolutionWindowMother::create(1_550_000_000, 1_560_000_000),
        Some(count(20)),
        DnsResolutionCreatedAt::from_unix_micros(1_560_000_100_000_000).expect("in range"),
    )
}

#[test]
fn it_exposes_the_report_and_the_merged_observation_as_primitives() {
    let now = DnsResolutionUpdatedAt::from_unix_micros(1_700_000_500_000_000).expect("in range");
    let reported_window = DnsResolutionWindowMother::create(1_700_000_000, 1_710_000_000);
    let (resolution, observation) = reported_in_2019()
        .observe(
            DnsResolutionSourceIdMother::create(SOURCE),
            reported_window,
            Some(count(12)),
            now,
        )
        .expect("a later window widens the observation");

    let primitives = create_dns_resolution_observed_event(
        &resolution,
        &observation,
        &reported_window,
        Some(count(12)),
    )
    .expect("event must be created")
    .to_primitives();

    assert_eq!(
        primitives.keys().collect::<Vec<_>>(),
        vec![
            "count",
            "first_seen",
            "id",
            "last_seen",
            "reported_count",
            "reported_first_seen",
            "reported_last_seen",
            "source_id",
            "updated_at"
        ]
    );
    assert_eq!(
        primitives["id"],
        PrimitiveValue::Text("2da44331-2011-5539-b65e-9ff44db9c139".to_string())
    );
    assert_eq!(
        primitives["source_id"],
        PrimitiveValue::Text(SOURCE.to_string())
    );
    assert_eq!(primitives["reported_first_seen"], seconds(1_700_000_000));
    assert_eq!(primitives["reported_last_seen"], seconds(1_710_000_000));
    assert_eq!(
        primitives["reported_count"],
        PrimitiveValue::UnsignedInteger(12)
    );
    assert_eq!(primitives["first_seen"], seconds(1_550_000_000));
    assert_eq!(primitives["last_seen"], seconds(1_710_000_000));
    assert_eq!(primitives["count"], PrimitiveValue::UnsignedInteger(20));
    assert_eq!(
        primitives["updated_at"],
        PrimitiveValue::Timestamp(now.value())
    );
}

#[test]
fn it_exposes_a_null_reported_count_when_the_source_reported_none() {
    let now = DnsResolutionUpdatedAt::from_unix_micros(1_700_000_500_000_000).expect("in range");
    let reported_window = DnsResolutionWindowMother::create(1_700_000_000, 1_710_000_000);
    let (resolution, observation) = reported_in_2019()
        .observe(
            DnsResolutionSourceIdMother::create(SOURCE),
            reported_window,
            None,
            now,
        )
        .expect("a later window widens the observation");

    let primitives =
        create_dns_resolution_observed_event(&resolution, &observation, &reported_window, None)
            .expect("event must be created")
            .to_primitives();

    assert_eq!(primitives["reported_count"], PrimitiveValue::Null);
    assert_eq!(primitives["count"], PrimitiveValue::UnsignedInteger(20));
}

#[test]
fn it_exposes_a_count_of_one_for_a_first_report_without_a_count() {
    let reported_window = DnsResolutionWindowMother::create(1_600_000_000, 1_650_000_000);
    let resolution = DnsResolutionMother::reported_by(
        DnsResolutionSubjectMother::create("example.com"),
        DnsResolutionAnswerMother::ipv4("93.184.216.34"),
        DnsResolutionSourceIdMother::create(SOURCE),
        reported_window,
    );

    let primitives = create_dns_resolution_observed_event(
        &resolution,
        &resolution.observations()[0],
        &reported_window,
        None,
    )
    .expect("event must be created")
    .to_primitives();

    assert_eq!(primitives["reported_first_seen"], seconds(1_600_000_000));
    assert_eq!(primitives["reported_last_seen"], seconds(1_650_000_000));
    assert_eq!(primitives["reported_count"], PrimitiveValue::Null);
    assert_eq!(primitives["first_seen"], seconds(1_600_000_000));
    assert_eq!(primitives["last_seen"], seconds(1_650_000_000));
    assert_eq!(primitives["count"], PrimitiveValue::UnsignedInteger(1));
    assert_eq!(
        primitives["updated_at"],
        PrimitiveValue::Timestamp(resolution.created_at().value())
    );
}

#[test]
fn it_is_named_after_the_kernel_dns_resolution_module() {
    let resolution = DnsResolutionMother::random();
    let observation = &resolution.observations()[0];

    let event =
        create_dns_resolution_observed_event(&resolution, observation, observation.window(), None)
            .expect("event must be created");

    assert_eq!(event.event_name(), "fennec.kernel.dns_resolution.observed");
    assert_eq!(event.event_name(), DnsResolutionObservedEvent::EVENT_NAME);
}

#[test]
fn it_uses_the_dns_resolution_id_as_aggregate_id() {
    let resolution = DnsResolutionMother::example();
    let observation = &resolution.observations()[0];

    let event =
        create_dns_resolution_observed_event(&resolution, observation, observation.window(), None)
            .expect("event must be created");

    assert_eq!(event.aggregate_id(), "2da44331-2011-5539-b65e-9ff44db9c139");
    assert_eq!(event.id(), resolution.id());
    assert_eq!(event.source_id(), observation.source_id());
}
