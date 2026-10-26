use std::sync::Arc;

use kernel::dns_resolution::application::retract_dns_resolution::dns_resolution_retracter::DnsResolutionRetracter;
use kernel::dns_resolution::domain::entities::dns_resolution::DnsResolution;
use kernel::dns_resolution::domain::errors::dns_resolution_repository_error::DnsResolutionRepositoryError;
use kernel::dns_resolution::domain::events::dns_resolution_deleted_event::DnsResolutionDeletedEvent;
use kernel::dns_resolution::domain::events::dns_resolution_retracted_event::DnsResolutionRetractedEvent;
use kernel::dns_resolution::domain::repositories::dns_resolution_repository::DnsResolutionRepository;
use kernel::dns_resolution::domain::value_objects::dns_resolution_source_id::DnsResolutionSourceId;
use kernel::dns_resolution::domain::value_objects::dns_resolution_updated_at::DnsResolutionUpdatedAt;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::dns_resolution::domain::entities::mothers::dns_resolution_mother::DnsResolutionMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_answer_mother::DnsResolutionAnswerMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_source_id_mother::DnsResolutionSourceIdMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_subject_mother::DnsResolutionSubjectMother;
use crate::src::dns_resolution::domain::value_objects::mothers::dns_resolution_window_mother::DnsResolutionWindowMother;
use crate::src::mocks::dns_resolution_repository_mock::DnsResolutionRepositoryMock;
use crate::src::mocks::event_bus_mock::EventBusMock;

fn make_retracter(
    repo: Arc<DnsResolutionRepositoryMock>,
    bus: Arc<EventBusMock>,
) -> DnsResolutionRetracter {
    let repo: Arc<dyn DnsResolutionRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    DnsResolutionRetracter::new(repo, bus)
}

/// A resolution observed by `first` and `second`.
fn observed_by_two(first: &DnsResolutionSourceId, second: &DnsResolutionSourceId) -> DnsResolution {
    let recorded = DnsResolutionMother::reported_by(
        DnsResolutionSubjectMother::random(),
        DnsResolutionAnswerMother::random(),
        first.clone(),
        DnsResolutionWindowMother::random(),
    );
    recorded
        .observe(
            second.clone(),
            DnsResolutionWindowMother::random(),
            None,
            DnsResolutionUpdatedAt::now(),
        )
        .expect("a new source always changes the resolution")
        .0
}

/// A resolution observed by `source_id` only.
fn observed_by_one(source_id: &DnsResolutionSourceId) -> DnsResolution {
    DnsResolutionMother::reported_by(
        DnsResolutionSubjectMother::random(),
        DnsResolutionAnswerMother::random(),
        source_id.clone(),
        DnsResolutionWindowMother::random(),
    )
}

// Other sources still assert the resolution

#[tokio::test]
async fn it_updates_the_resolution_without_the_retracted_observation() {
    let retracted = DnsResolutionSourceIdMother::random();
    let kept = DnsResolutionSourceIdMother::random();
    let dns_resolution = observed_by_two(&retracted, &kept);
    let repo = Arc::new(DnsResolutionRepositoryMock::that_returns_dns_resolution(
        dns_resolution.clone(),
    ));
    let bus = Arc::new(EventBusMock::new());
    let retracter = make_retracter(repo.clone(), bus.clone());

    retracter
        .execute(*dns_resolution.id(), retracted.clone())
        .await
        .unwrap();

    let updated = repo.updated();
    assert_eq!(updated.len(), 1);
    assert_eq!(updated[0].id(), dns_resolution.id());
    assert_eq!(updated[0].observations().len(), 1);
    assert!(updated[0].observation(&retracted).is_none());
    assert_eq!(
        updated[0].observation(&kept),
        dns_resolution.observation(&kept)
    );
    assert!(repo.deleted_ids().is_empty());
}

#[tokio::test]
async fn it_publishes_a_retracted_event_when_other_sources_remain() {
    let retracted = DnsResolutionSourceIdMother::random();
    let dns_resolution = observed_by_two(&retracted, &DnsResolutionSourceIdMother::random());
    let repo = Arc::new(DnsResolutionRepositoryMock::that_returns_dns_resolution(
        dns_resolution.clone(),
    ));
    let bus = Arc::new(EventBusMock::new());
    let retracter = make_retracter(repo.clone(), bus.clone());

    retracter
        .execute(*dns_resolution.id(), retracted)
        .await
        .unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![DnsResolutionRetractedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_does_not_publish_when_update_fails() {
    let retracted = DnsResolutionSourceIdMother::random();
    let dns_resolution = observed_by_two(&retracted, &DnsResolutionSourceIdMother::random());
    let repo = Arc::new(
        DnsResolutionRepositoryMock::that_returns_dns_resolution_but_update_fails(
            dns_resolution.clone(),
            "storage error".to_string(),
        ),
    );
    let bus = Arc::new(EventBusMock::new());
    let retracter = make_retracter(repo.clone(), bus.clone());

    let result = retracter.execute(*dns_resolution.id(), retracted).await;

    assert!(matches!(
        result,
        Err(DnsResolutionRepositoryError::Unexpected(_))
    ));
    assert!(bus.published_event_names().is_empty());
}

#[tokio::test]
async fn it_returns_an_unexpected_error_when_publishing_a_retraction_fails() {
    let retracted = DnsResolutionSourceIdMother::random();
    let dns_resolution = observed_by_two(&retracted, &DnsResolutionSourceIdMother::random());
    let repo = Arc::new(DnsResolutionRepositoryMock::that_returns_dns_resolution(
        dns_resolution.clone(),
    ));
    let bus = Arc::new(EventBusMock::that_fails());
    let retracter = make_retracter(repo.clone(), bus.clone());

    let result = retracter.execute(*dns_resolution.id(), retracted).await;

    assert!(matches!(
        result,
        Err(DnsResolutionRepositoryError::Unexpected(_))
    ));
    assert_eq!(repo.updated().len(), 1);
    assert!(repo.deleted_ids().is_empty());
}

// The last observation is retracted

#[tokio::test]
async fn it_deletes_the_resolution_when_the_last_observation_is_retracted() {
    let source_id = DnsResolutionSourceIdMother::random();
    let dns_resolution = observed_by_one(&source_id);
    let repo = Arc::new(DnsResolutionRepositoryMock::that_returns_dns_resolution(
        dns_resolution.clone(),
    ));
    let bus = Arc::new(EventBusMock::new());
    let retracter = make_retracter(repo.clone(), bus.clone());

    retracter
        .execute(*dns_resolution.id(), source_id)
        .await
        .unwrap();

    assert_eq!(repo.deleted_ids(), vec![*dns_resolution.id()]);
    assert!(repo.updated().is_empty());
}

#[tokio::test]
async fn it_publishes_a_retracted_then_a_deleted_event_when_the_last_observation_is_retracted() {
    let source_id = DnsResolutionSourceIdMother::random();
    let dns_resolution = observed_by_one(&source_id);
    let repo = Arc::new(DnsResolutionRepositoryMock::that_returns_dns_resolution(
        dns_resolution.clone(),
    ));
    let bus = Arc::new(EventBusMock::new());
    let retracter = make_retracter(repo.clone(), bus.clone());

    retracter
        .execute(*dns_resolution.id(), source_id)
        .await
        .unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![
            DnsResolutionRetractedEvent::EVENT_NAME,
            DnsResolutionDeletedEvent::EVENT_NAME
        ]
    );
}

#[tokio::test]
async fn it_does_not_publish_when_delete_fails() {
    let source_id = DnsResolutionSourceIdMother::random();
    let dns_resolution = observed_by_one(&source_id);
    let repo = Arc::new(
        DnsResolutionRepositoryMock::that_returns_dns_resolution_but_delete_fails(
            dns_resolution.clone(),
            "storage error".to_string(),
        ),
    );
    let bus = Arc::new(EventBusMock::new());
    let retracter = make_retracter(repo.clone(), bus.clone());

    let result = retracter.execute(*dns_resolution.id(), source_id).await;

    assert!(matches!(
        result,
        Err(DnsResolutionRepositoryError::Unexpected(_))
    ));
    assert!(bus.published_event_names().is_empty());
}

#[tokio::test]
async fn it_returns_an_unexpected_error_when_publishing_the_retraction_of_the_last_observation_fails(
) {
    let source_id = DnsResolutionSourceIdMother::random();
    let dns_resolution = observed_by_one(&source_id);
    let repo = Arc::new(DnsResolutionRepositoryMock::that_returns_dns_resolution(
        dns_resolution.clone(),
    ));
    let bus = Arc::new(EventBusMock::that_fails());
    let retracter = make_retracter(repo.clone(), bus.clone());

    let result = retracter.execute(*dns_resolution.id(), source_id).await;

    assert!(matches!(
        result,
        Err(DnsResolutionRepositoryError::Unexpected(_))
    ));
}

// Nothing to retract

#[tokio::test]
async fn it_returns_not_found_when_the_source_has_not_observed_the_resolution() {
    let dns_resolution = observed_by_one(&DnsResolutionSourceIdMother::random());
    let repo = Arc::new(DnsResolutionRepositoryMock::that_returns_dns_resolution(
        dns_resolution.clone(),
    ));
    let bus = Arc::new(EventBusMock::new());
    let retracter = make_retracter(repo.clone(), bus.clone());

    let result = retracter
        .execute(*dns_resolution.id(), DnsResolutionSourceIdMother::random())
        .await;

    assert!(matches!(
        result,
        Err(DnsResolutionRepositoryError::NotFound)
    ));
    assert!(repo.updated().is_empty());
    assert!(repo.deleted_ids().is_empty());
    assert!(bus.published_event_names().is_empty());
}

#[tokio::test]
async fn it_returns_not_found_when_the_resolution_does_not_exist() {
    let repo = Arc::new(DnsResolutionRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let retracter = make_retracter(repo.clone(), bus.clone());

    let result = retracter
        .execute(
            *DnsResolutionMother::random().id(),
            DnsResolutionSourceIdMother::random(),
        )
        .await;

    assert!(matches!(
        result,
        Err(DnsResolutionRepositoryError::NotFound)
    ));
    assert!(repo.updated().is_empty());
    assert!(repo.deleted_ids().is_empty());
    assert!(bus.published_event_names().is_empty());
}

#[tokio::test]
async fn it_returns_an_error_and_writes_nothing_when_the_lookup_fails() {
    let repo = Arc::new(DnsResolutionRepositoryMock::that_fails_on_find(
        "storage error".to_string(),
    ));
    let bus = Arc::new(EventBusMock::new());
    let retracter = make_retracter(repo.clone(), bus.clone());

    let result = retracter
        .execute(
            *DnsResolutionMother::random().id(),
            DnsResolutionSourceIdMother::random(),
        )
        .await;

    assert!(matches!(
        result,
        Err(DnsResolutionRepositoryError::Unexpected(_))
    ));
    assert!(repo.updated().is_empty());
    assert!(repo.deleted_ids().is_empty());
    assert!(bus.published_event_names().is_empty());
}
