use std::sync::Arc;

use kernel::ioc::application::delete_ioc::ioc_deleter::IocDeleter;
use kernel::ioc::domain::errors::ioc_repository_error::IocRepositoryError;
use kernel::ioc::domain::events::ioc_deleted_event::IocDeletedEvent;
use kernel::ioc::domain::repositories::ioc_repository::IocRepository;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::ioc::domain::entities::mothers::ioc_mother::IocMother;
use crate::src::ioc::domain::value_objects::mothers::ioc_id_mother::IocIdMother;
use crate::src::mocks::event_bus_mock::EventBusMock;
use crate::src::mocks::ioc_repository_mock::IocRepositoryMock;

fn make_deleter(repo: Arc<IocRepositoryMock>, bus: Arc<EventBusMock>) -> IocDeleter {
    let repo: Arc<dyn IocRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    IocDeleter::new(repo, bus)
}

#[tokio::test]
async fn it_calls_delete_on_the_repository() {
    let ioc = IocMother::random();
    let repo = Arc::new(IocRepositoryMock::that_returns_ioc(ioc));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    deleter.execute(IocIdMother::random()).await.unwrap();

    assert_eq!(repo.delete_call_count(), 1);
}

#[tokio::test]
async fn it_publishes_a_deleted_event() {
    let ioc = IocMother::random();
    let repo = Arc::new(IocRepositoryMock::that_returns_ioc(ioc));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    deleter.execute(IocIdMother::random()).await.unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![IocDeletedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_not_found_when_ioc_does_not_exist() {
    let repo = Arc::new(IocRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let result = deleter.execute(IocIdMother::random()).await;

    assert!(matches!(result, Err(IocRepositoryError::NotFound)));
}

#[tokio::test]
async fn it_does_not_publish_event_when_ioc_not_found() {
    let repo = Arc::new(IocRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let _ = deleter.execute(IocIdMother::random()).await;

    assert!(bus.published_event_names().is_empty());
}
