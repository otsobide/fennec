use std::sync::Arc;

use kernel::dns_resolution::application::delete_dns_resolution::dns_resolution_deleter::DnsResolutionDeleter;
use kernel::dns_resolution::domain::errors::dns_resolution_repository_error::DnsResolutionRepositoryError;
use kernel::dns_resolution::domain::events::dns_resolution_deleted_event::DnsResolutionDeletedEvent;
use kernel::dns_resolution::domain::repositories::dns_resolution_repository::DnsResolutionRepository;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::dns_resolution::domain::entities::mothers::dns_resolution_mother::DnsResolutionMother;
use crate::src::mocks::dns_resolution_repository_mock::DnsResolutionRepositoryMock;
use crate::src::mocks::event_bus_mock::EventBusMock;

fn make_deleter(
    repo: Arc<DnsResolutionRepositoryMock>,
    bus: Arc<EventBusMock>,
) -> DnsResolutionDeleter {
    let repo: Arc<dyn DnsResolutionRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    DnsResolutionDeleter::new(repo, bus)
}

#[tokio::test]
async fn it_deletes_the_resolution() {
    let dns_resolution = DnsResolutionMother::random();
    let id = *dns_resolution.id();
    let repo = Arc::new(DnsResolutionRepositoryMock::that_returns_dns_resolution(
        dns_resolution,
    ));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    deleter.execute(id).await.unwrap();

    assert_eq!(repo.deleted_ids(), vec![id]);
}

#[tokio::test]
async fn it_publishes_a_deleted_event() {
    let dns_resolution = DnsResolutionMother::random();
    let id = *dns_resolution.id();
    let repo = Arc::new(DnsResolutionRepositoryMock::that_returns_dns_resolution(
        dns_resolution,
    ));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    deleter.execute(id).await.unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![DnsResolutionDeletedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_not_found_when_the_resolution_does_not_exist() {
    let repo = Arc::new(DnsResolutionRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let result = deleter.execute(*DnsResolutionMother::random().id()).await;

    assert!(matches!(
        result,
        Err(DnsResolutionRepositoryError::NotFound)
    ));
}

#[tokio::test]
async fn it_does_not_delete_or_publish_when_the_resolution_does_not_exist() {
    let repo = Arc::new(DnsResolutionRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let _ = deleter.execute(*DnsResolutionMother::random().id()).await;

    assert!(repo.deleted_ids().is_empty());
    assert!(bus.published_event_names().is_empty());
}

#[tokio::test]
async fn it_does_not_publish_when_delete_fails() {
    let dns_resolution = DnsResolutionMother::random();
    let id = *dns_resolution.id();
    let repo = Arc::new(
        DnsResolutionRepositoryMock::that_returns_dns_resolution_but_delete_fails(
            dns_resolution,
            "storage error".to_string(),
        ),
    );
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let result = deleter.execute(id).await;

    assert!(matches!(
        result,
        Err(DnsResolutionRepositoryError::Unexpected(_))
    ));
    assert!(bus.published_event_names().is_empty());
}

#[tokio::test]
async fn it_returns_an_error_and_deletes_nothing_when_the_lookup_fails() {
    let repo = Arc::new(DnsResolutionRepositoryMock::that_fails_on_find(
        "storage error".to_string(),
    ));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let result = deleter.execute(*DnsResolutionMother::random().id()).await;

    assert!(matches!(
        result,
        Err(DnsResolutionRepositoryError::Unexpected(_))
    ));
    assert!(repo.deleted_ids().is_empty());
    assert!(bus.published_event_names().is_empty());
}

#[tokio::test]
async fn it_returns_an_unexpected_error_when_publishing_fails() {
    let dns_resolution = DnsResolutionMother::random();
    let id = *dns_resolution.id();
    let repo = Arc::new(DnsResolutionRepositoryMock::that_returns_dns_resolution(
        dns_resolution,
    ));
    let bus = Arc::new(EventBusMock::that_fails());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let result = deleter.execute(id).await;

    assert!(matches!(
        result,
        Err(DnsResolutionRepositoryError::Unexpected(_))
    ));
}
