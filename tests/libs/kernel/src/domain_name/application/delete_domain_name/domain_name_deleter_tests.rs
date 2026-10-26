use std::sync::Arc;

use kernel::domain_name::application::delete_domain_name::domain_name_deleter::DomainNameDeleter;
use kernel::domain_name::domain::errors::domain_name_repository_error::DomainNameRepositoryError;
use kernel::domain_name::domain::events::domain_name_deleted_event::DomainNameDeletedEvent;
use kernel::domain_name::domain::repositories::domain_name_repository::DomainNameRepository;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::domain_name::domain::entities::mothers::domain_name_mother::DomainNameMother;
use crate::src::domain_name::domain::value_objects::mothers::domain_name_id_mother::DomainNameIdMother;
use crate::src::mocks::domain_name_repository_mock::DomainNameRepositoryMock;
use crate::src::mocks::event_bus_mock::EventBusMock;

fn make_deleter(repo: Arc<DomainNameRepositoryMock>, bus: Arc<EventBusMock>) -> DomainNameDeleter {
    let repo: Arc<dyn DomainNameRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    DomainNameDeleter::new(repo, bus)
}

#[tokio::test]
async fn it_deletes_the_domain_name() {
    let domain_name = DomainNameMother::random();
    let id = *domain_name.id();
    let repo = Arc::new(DomainNameRepositoryMock::that_returns_domain_name(
        domain_name,
    ));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    deleter.execute(id).await.unwrap();

    assert_eq!(repo.delete_call_count(), 1);
}

#[tokio::test]
async fn it_publishes_a_deleted_event() {
    let domain_name = DomainNameMother::random();
    let id = *domain_name.id();
    let repo = Arc::new(DomainNameRepositoryMock::that_returns_domain_name(
        domain_name,
    ));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    deleter.execute(id).await.unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![DomainNameDeletedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_not_found_when_the_domain_name_does_not_exist() {
    let repo = Arc::new(DomainNameRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let result = deleter.execute(DomainNameIdMother::random()).await;

    assert!(matches!(result, Err(DomainNameRepositoryError::NotFound)));
}

#[tokio::test]
async fn it_does_not_delete_or_publish_when_the_domain_name_does_not_exist() {
    let repo = Arc::new(DomainNameRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let _ = deleter.execute(DomainNameIdMother::random()).await;

    assert_eq!(repo.delete_call_count(), 0);
    assert!(bus.published_event_names().is_empty());
}

#[tokio::test]
async fn it_does_not_publish_event_when_delete_fails() {
    let domain_name = DomainNameMother::random();
    let id = *domain_name.id();
    let repo = Arc::new(
        DomainNameRepositoryMock::that_returns_domain_name_but_delete_fails(
            domain_name,
            "storage error".to_string(),
        ),
    );
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let result = deleter.execute(id).await;

    assert!(matches!(
        result,
        Err(DomainNameRepositoryError::Unexpected(_))
    ));
    assert!(bus.published_event_names().is_empty());
}
