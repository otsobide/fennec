use std::sync::Arc;
use std::time::{Duration, UNIX_EPOCH};

use kernel::dns_resolution::application::record_dns_resolution::dns_resolution_recorder::DnsResolutionRecorder;
use kernel::dns_resolution::application::record_dns_resolution::dns_resolution_recording::DnsResolutionRecording;
use kernel::dns_resolution::domain::errors::dns_resolution_repository_error::DnsResolutionRepositoryError;
use kernel::dns_resolution::domain::events::dns_resolution_created_event::DnsResolutionCreatedEvent;
use kernel::dns_resolution::domain::events::dns_resolution_observed_event::DnsResolutionObservedEvent;
use kernel::dns_resolution::domain::repositories::dns_resolution_repository::DnsResolutionRepository;
use kernel::dns_resolution::domain::value_objects::dns_resolution_count::DnsResolutionCount;
use kernel::dns_resolution::domain::value_objects::dns_resolution_id::DnsResolutionId;
use shared_domain_events::domain::event_bus::EventBus;
use shared_domain_events::domain::primitive_value::PrimitiveValue;

use crate::src::dns_resolution::domain::entities::mothers::dns_resolution_mother::DnsResolutionMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_answer_mother::DnsResolutionAnswerMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_source_id_mother::DnsResolutionSourceIdMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_subject_mother::DnsResolutionSubjectMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_window_mother::DnsResolutionWindowMother;
use crate::src::mocks::dns_resolution_repository_mock::DnsResolutionRepositoryMock;
use crate::src::mocks::event_bus_mock::EventBusMock;

fn make_recorder(
    repo: Arc<DnsResolutionRepositoryMock>,
    bus: Arc<EventBusMock>,
) -> DnsResolutionRecorder {
    let repo: Arc<dyn DnsResolutionRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    DnsResolutionRecorder::new(repo, bus)
}

fn count(value: u64) -> DnsResolutionCount {
    DnsResolutionCount::from_u64(value).unwrap()
}

fn seconds(value: u64) -> PrimitiveValue {
    PrimitiveValue::Timestamp(UNIX_EPOCH + Duration::from_secs(value))
}

// A resolution recorded for the first time

#[tokio::test]
async fn it_looks_up_the_id_derived_from_the_endpoints() {
    let subject = DnsResolutionSubjectMother::random();
    let answer = DnsResolutionAnswerMother::random();
    let repo = Arc::new(DnsResolutionRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let recorder = make_recorder(repo.clone(), bus.clone());

    recorder
        .execute(
            subject.clone(),
            answer,
            DnsResolutionSourceIdMother::random(),
            DnsResolutionWindowMother::random(),
            None,
        )
        .await
        .unwrap();

    assert_eq!(
        repo.found_ids(),
        vec![DnsResolutionId::from_endpoints(&subject, &answer)]
    );
}

#[tokio::test]
async fn it_saves_a_new_resolution_with_the_report_as_its_first_observation() {
    let subject = DnsResolutionSubjectMother::random();
    let answer = DnsResolutionAnswerMother::random();
    let source_id = DnsResolutionSourceIdMother::random();
    let window = DnsResolutionWindowMother::random();
    let repo = Arc::new(DnsResolutionRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let recorder = make_recorder(repo.clone(), bus.clone());

    recorder
        .execute(
            subject.clone(),
            answer,
            source_id.clone(),
            window,
            Some(count(7)),
        )
        .await
        .unwrap();

    let saved = repo.saved();
    assert_eq!(saved.len(), 1);
    assert_eq!(saved[0].subject(), &subject);
    assert_eq!(saved[0].answer(), &answer);
    assert_eq!(saved[0].observations().len(), 1);
    let observation = &saved[0].observations()[0];
    assert_eq!(observation.source_id(), &source_id);
    assert_eq!(observation.window(), &window);
    assert_eq!(observation.count(), count(7));
    assert!(repo.updated().is_empty());
}

#[tokio::test]
async fn it_returns_created_with_the_derived_id() {
    let subject = DnsResolutionSubjectMother::create("example.com");
    let answer = DnsResolutionAnswerMother::ipv4("93.184.216.34");
    let repo = Arc::new(DnsResolutionRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let recorder = make_recorder(repo.clone(), bus.clone());

    let recording = recorder
        .execute(
            subject,
            answer,
            DnsResolutionSourceIdMother::random(),
            DnsResolutionWindowMother::random(),
            None,
        )
        .await
        .unwrap();

    assert_eq!(
        recording,
        DnsResolutionRecording::Created(
            DnsResolutionId::new("2da44331-2011-5539-b65e-9ff44db9c139").unwrap()
        )
    );
    assert_eq!(recording.outcome(), "created");
}

#[tokio::test]
async fn it_publishes_a_created_then_an_observed_event_for_a_new_resolution() {
    let repo = Arc::new(DnsResolutionRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let recorder = make_recorder(repo.clone(), bus.clone());

    recorder
        .execute(
            DnsResolutionSubjectMother::random(),
            DnsResolutionAnswerMother::random(),
            DnsResolutionSourceIdMother::random(),
            DnsResolutionWindowMother::random(),
            None,
        )
        .await
        .unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![
            DnsResolutionCreatedEvent::EVENT_NAME,
            DnsResolutionObservedEvent::EVENT_NAME
        ]
    );
}

#[tokio::test]
async fn it_publishes_the_reported_window_and_count_of_the_first_observation() {
    let repo = Arc::new(DnsResolutionRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let recorder = make_recorder(repo.clone(), bus.clone());

    recorder
        .execute(
            DnsResolutionSubjectMother::random(),
            DnsResolutionAnswerMother::random(),
            DnsResolutionSourceIdMother::random(),
            DnsResolutionWindowMother::create(1_600_000_000, 1_650_000_000),
            Some(count(7)),
        )
        .await
        .unwrap();

    let observed = &bus.published_primitives()[1];
    assert_eq!(observed["reported_first_seen"], seconds(1_600_000_000));
    assert_eq!(observed["reported_last_seen"], seconds(1_650_000_000));
    assert_eq!(
        observed["reported_count"],
        PrimitiveValue::UnsignedInteger(7)
    );
    assert_eq!(observed["first_seen"], seconds(1_600_000_000));
    assert_eq!(observed["last_seen"], seconds(1_650_000_000));
    assert_eq!(observed["count"], PrimitiveValue::UnsignedInteger(7));
}

#[tokio::test]
async fn it_returns_already_exists_and_publishes_nothing_when_save_finds_it_recorded() {
    let repo = Arc::new(DnsResolutionRepositoryMock::that_fails_with_already_exists());
    let bus = Arc::new(EventBusMock::new());
    let recorder = make_recorder(repo.clone(), bus.clone());

    let result = recorder
        .execute(
            DnsResolutionSubjectMother::random(),
            DnsResolutionAnswerMother::random(),
            DnsResolutionSourceIdMother::random(),
            DnsResolutionWindowMother::random(),
            None,
        )
        .await;

    assert!(matches!(
        result,
        Err(DnsResolutionRepositoryError::AlreadyExists)
    ));
    assert!(bus.published_event_names().is_empty());
}

#[tokio::test]
async fn it_does_not_publish_when_save_fails() {
    let repo = Arc::new(DnsResolutionRepositoryMock::that_fails_on_save(
        "storage error".to_string(),
    ));
    let bus = Arc::new(EventBusMock::new());
    let recorder = make_recorder(repo.clone(), bus.clone());

    let result = recorder
        .execute(
            DnsResolutionSubjectMother::random(),
            DnsResolutionAnswerMother::random(),
            DnsResolutionSourceIdMother::random(),
            DnsResolutionWindowMother::random(),
            None,
        )
        .await;

    assert!(matches!(
        result,
        Err(DnsResolutionRepositoryError::Unexpected(_))
    ));
    assert!(bus.published_event_names().is_empty());
}

#[tokio::test]
async fn it_writes_nothing_and_publishes_nothing_when_the_lookup_fails() {
    let repo = Arc::new(DnsResolutionRepositoryMock::that_fails_on_find(
        "storage error".to_string(),
    ));
    let bus = Arc::new(EventBusMock::new());
    let recorder = make_recorder(repo.clone(), bus.clone());

    let result = recorder
        .execute(
            DnsResolutionSubjectMother::random(),
            DnsResolutionAnswerMother::random(),
            DnsResolutionSourceIdMother::random(),
            DnsResolutionWindowMother::random(),
            None,
        )
        .await;

    assert!(matches!(
        result,
        Err(DnsResolutionRepositoryError::Unexpected(_))
    ));
    assert!(repo.saved().is_empty());
    assert!(repo.updated().is_empty());
    assert!(bus.published_event_names().is_empty());
}

#[tokio::test]
async fn it_returns_an_unexpected_error_when_publishing_fails() {
    let repo = Arc::new(DnsResolutionRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::that_fails());
    let recorder = make_recorder(repo.clone(), bus.clone());

    let result = recorder
        .execute(
            DnsResolutionSubjectMother::random(),
            DnsResolutionAnswerMother::random(),
            DnsResolutionSourceIdMother::random(),
            DnsResolutionWindowMother::random(),
            None,
        )
        .await;

    assert!(matches!(
        result,
        Err(DnsResolutionRepositoryError::Unexpected(_))
    ));
}

// A resolution already recorded

#[tokio::test]
async fn it_adds_the_observation_of_a_new_source_to_a_recorded_resolution() {
    let recorded = DnsResolutionMother::random();
    let source_id = DnsResolutionSourceIdMother::random();
    let window = DnsResolutionWindowMother::random();
    let repo = Arc::new(DnsResolutionRepositoryMock::that_returns_dns_resolution(
        recorded.clone(),
    ));
    let bus = Arc::new(EventBusMock::new());
    let recorder = make_recorder(repo.clone(), bus.clone());

    let recording = recorder
        .execute(
            recorded.subject().clone(),
            *recorded.answer(),
            source_id.clone(),
            window,
            None,
        )
        .await
        .unwrap();

    assert_eq!(recording, DnsResolutionRecording::Observed(*recorded.id()));
    assert_eq!(recording.outcome(), "observed");
    assert!(repo.saved().is_empty());
    let updated = repo.updated();
    assert_eq!(updated.len(), 1);
    assert_eq!(updated[0].id(), recorded.id());
    assert_eq!(updated[0].created_at(), recorded.created_at());
    assert_eq!(updated[0].observations().len(), 2);
    assert_eq!(
        updated[0].observation(&source_id).unwrap().window(),
        &window
    );
}

#[tokio::test]
async fn it_widens_the_window_of_a_known_source() {
    let source_id = DnsResolutionSourceIdMother::random();
    let recorded = DnsResolutionMother::reported_by(
        DnsResolutionSubjectMother::random(),
        DnsResolutionAnswerMother::random(),
        source_id.clone(),
        DnsResolutionWindowMother::create(1_600_000_000, 1_600_000_100),
    );
    let repo = Arc::new(DnsResolutionRepositoryMock::that_returns_dns_resolution(
        recorded.clone(),
    ));
    let bus = Arc::new(EventBusMock::new());
    let recorder = make_recorder(repo.clone(), bus.clone());

    let recording = recorder
        .execute(
            recorded.subject().clone(),
            *recorded.answer(),
            source_id.clone(),
            DnsResolutionWindowMother::create(1_600_000_050, 1_600_000_200),
            None,
        )
        .await
        .unwrap();

    assert_eq!(recording, DnsResolutionRecording::Observed(*recorded.id()));
    let updated = repo.updated();
    assert_eq!(updated.len(), 1);
    assert_eq!(updated[0].observations().len(), 1);
    assert_eq!(
        updated[0].observation(&source_id).unwrap().window(),
        &DnsResolutionWindowMother::create(1_600_000_000, 1_600_000_200)
    );
}

#[tokio::test]
async fn it_keeps_the_larger_count_of_a_known_source() {
    let source_id = DnsResolutionSourceIdMother::random();
    let window = DnsResolutionWindowMother::random();
    let recorded = DnsResolutionMother::reported_by(
        DnsResolutionSubjectMother::random(),
        DnsResolutionAnswerMother::random(),
        source_id.clone(),
        window,
    );
    let repo = Arc::new(DnsResolutionRepositoryMock::that_returns_dns_resolution(
        recorded.clone(),
    ));
    let bus = Arc::new(EventBusMock::new());
    let recorder = make_recorder(repo.clone(), bus.clone());

    recorder
        .execute(
            recorded.subject().clone(),
            *recorded.answer(),
            source_id.clone(),
            window,
            Some(count(42)),
        )
        .await
        .unwrap();

    let updated = repo.updated();
    assert_eq!(updated.len(), 1);
    assert_eq!(
        updated[0].observation(&source_id).unwrap().count(),
        count(42)
    );
}

#[tokio::test]
async fn it_publishes_only_an_observed_event_for_a_recorded_resolution() {
    let recorded = DnsResolutionMother::random();
    let repo = Arc::new(DnsResolutionRepositoryMock::that_returns_dns_resolution(
        recorded.clone(),
    ));
    let bus = Arc::new(EventBusMock::new());
    let recorder = make_recorder(repo.clone(), bus.clone());

    recorder
        .execute(
            recorded.subject().clone(),
            *recorded.answer(),
            DnsResolutionSourceIdMother::random(),
            DnsResolutionWindowMother::random(),
            None,
        )
        .await
        .unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![DnsResolutionObservedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_publishes_the_reported_window_next_to_the_merged_one() {
    let source_id = DnsResolutionSourceIdMother::random();
    let recorded = DnsResolutionMother::reported_by(
        DnsResolutionSubjectMother::random(),
        DnsResolutionAnswerMother::random(),
        source_id.clone(),
        DnsResolutionWindowMother::create(1_550_000_000, 1_560_000_000),
    );
    let repo = Arc::new(DnsResolutionRepositoryMock::that_returns_dns_resolution(
        recorded.clone(),
    ));
    let bus = Arc::new(EventBusMock::new());
    let recorder = make_recorder(repo.clone(), bus.clone());

    recorder
        .execute(
            recorded.subject().clone(),
            *recorded.answer(),
            source_id,
            DnsResolutionWindowMother::create(1_700_000_000, 1_710_000_000),
            None,
        )
        .await
        .unwrap();

    let published = bus.published_primitives();
    assert_eq!(published.len(), 1);
    let observed = &published[0];
    assert_eq!(observed["reported_first_seen"], seconds(1_700_000_000));
    assert_eq!(observed["reported_last_seen"], seconds(1_710_000_000));
    assert_eq!(observed["reported_count"], PrimitiveValue::Null);
    assert_eq!(observed["first_seen"], seconds(1_550_000_000));
    assert_eq!(observed["last_seen"], seconds(1_710_000_000));
    assert_eq!(observed["count"], PrimitiveValue::UnsignedInteger(1));
}

#[tokio::test]
async fn it_writes_and_publishes_nothing_when_the_report_adds_nothing() {
    let source_id = DnsResolutionSourceIdMother::random();
    let window = DnsResolutionWindowMother::create(1_600_000_000, 1_600_000_100);
    let recorded = DnsResolutionMother::reported_by(
        DnsResolutionSubjectMother::random(),
        DnsResolutionAnswerMother::random(),
        source_id.clone(),
        window,
    );
    let repo = Arc::new(DnsResolutionRepositoryMock::that_returns_dns_resolution(
        recorded.clone(),
    ));
    let bus = Arc::new(EventBusMock::new());
    let recorder = make_recorder(repo.clone(), bus.clone());

    let recording = recorder
        .execute(
            recorded.subject().clone(),
            *recorded.answer(),
            source_id,
            DnsResolutionWindowMother::create(1_600_000_010, 1_600_000_090),
            Some(count(1)),
        )
        .await
        .unwrap();

    assert_eq!(recording, DnsResolutionRecording::Unchanged(*recorded.id()));
    assert_eq!(recording.outcome(), "unchanged");
    assert!(repo.saved().is_empty());
    assert!(repo.updated().is_empty());
    assert!(bus.published_event_names().is_empty());
}

#[tokio::test]
async fn it_returns_unchanged_when_the_same_report_is_sent_twice() {
    let source_id = DnsResolutionSourceIdMother::random();
    let window = DnsResolutionWindowMother::random();
    let recorded = DnsResolutionMother::reported_by(
        DnsResolutionSubjectMother::random(),
        DnsResolutionAnswerMother::random(),
        source_id.clone(),
        window,
    );
    let repo = Arc::new(DnsResolutionRepositoryMock::that_returns_dns_resolution(
        recorded.clone(),
    ));
    let bus = Arc::new(EventBusMock::new());
    let recorder = make_recorder(repo.clone(), bus.clone());

    let recording = recorder
        .execute(
            recorded.subject().clone(),
            *recorded.answer(),
            source_id,
            window,
            None,
        )
        .await
        .unwrap();

    assert_eq!(recording, DnsResolutionRecording::Unchanged(*recorded.id()));
    assert!(repo.updated().is_empty());
    assert!(bus.published_event_names().is_empty());
}

#[tokio::test]
async fn it_does_not_publish_when_update_fails() {
    let recorded = DnsResolutionMother::random();
    let repo = Arc::new(
        DnsResolutionRepositoryMock::that_returns_dns_resolution_but_update_fails(
            recorded.clone(),
            "storage error".to_string(),
        ),
    );
    let bus = Arc::new(EventBusMock::new());
    let recorder = make_recorder(repo.clone(), bus.clone());

    let result = recorder
        .execute(
            recorded.subject().clone(),
            *recorded.answer(),
            DnsResolutionSourceIdMother::random(),
            DnsResolutionWindowMother::random(),
            None,
        )
        .await;

    assert!(matches!(
        result,
        Err(DnsResolutionRepositoryError::Unexpected(_))
    ));
    assert!(bus.published_event_names().is_empty());
}

#[tokio::test]
async fn it_returns_an_unexpected_error_when_publishing_an_observation_fails() {
    let recorded = DnsResolutionMother::random();
    let repo = Arc::new(DnsResolutionRepositoryMock::that_returns_dns_resolution(
        recorded.clone(),
    ));
    let bus = Arc::new(EventBusMock::that_fails());
    let recorder = make_recorder(repo.clone(), bus.clone());

    let result = recorder
        .execute(
            recorded.subject().clone(),
            *recorded.answer(),
            DnsResolutionSourceIdMother::random(),
            DnsResolutionWindowMother::random(),
            None,
        )
        .await;

    assert!(matches!(
        result,
        Err(DnsResolutionRepositoryError::Unexpected(_))
    ));
}
