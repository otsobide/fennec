use std::sync::Arc;

use ioc::ioc::application::create_ioc::ioc_creator::IocCreator;
use ioc::ioc::domain::errors::ioc_repository_error::IocRepositoryError;
use ioc::ioc::domain::events::ioc_created_event::IocCreatedEvent;
use ioc::ioc::domain::repositories::ioc_repository::IocRepository;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::ioc::domain::value_objects::mothers::ioc_id_mother::IocIdMother;
use crate::src::ioc::domain::value_objects::mothers::ioc_type_mother::IocTypeMother;
use crate::src::ioc::domain::value_objects::mothers::ioc_value_mother::IocValueMother;
use crate::src::mocks::event_bus_mock::EventBusMock;
use crate::src::mocks::ioc_repository_mock::IocRepositoryMock;

fn make_creator(repo: Arc<IocRepositoryMock>, bus: Arc<EventBusMock>) -> IocCreator {
    let repo: Arc<dyn IocRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    IocCreator::new(repo, bus)
}

#[tokio::test]
async fn it_saves_the_ioc() {
    let id = IocIdMother::random();
    let expected_id = *id.value();

    let repo = Arc::new(IocRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    creator
        .execute(id, IocTypeMother::random(), IocValueMother::random())
        .await
        .unwrap();

    assert_eq!(repo.saved_ids(), vec![expected_id]);
}

#[tokio::test]
async fn it_publishes_a_created_event() {
    let repo = Arc::new(IocRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    creator
        .execute(
            IocIdMother::random(),
            IocTypeMother::random(),
            IocValueMother::random(),
        )
        .await
        .unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![IocCreatedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_already_exists_error() {
    let repo = Arc::new(IocRepositoryMock::that_fails_with_already_exists());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator
        .execute(
            IocIdMother::random(),
            IocTypeMother::random(),
            IocValueMother::random(),
        )
        .await;

    assert!(matches!(result, Err(IocRepositoryError::AlreadyExists)));
}

#[tokio::test]
async fn it_does_not_publish_event_when_save_fails() {
    let repo = Arc::new(IocRepositoryMock::that_fails_with_already_exists());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let _ = creator
        .execute(
            IocIdMother::random(),
            IocTypeMother::random(),
            IocValueMother::random(),
        )
        .await;

    assert!(bus.published_event_names().is_empty());
}
