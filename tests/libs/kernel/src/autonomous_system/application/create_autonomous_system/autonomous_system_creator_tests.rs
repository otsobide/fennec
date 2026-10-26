use std::sync::Arc;

use kernel::autonomous_system::application::create_autonomous_system::autonomous_system_creator::AutonomousSystemCreator;
use kernel::autonomous_system::domain::errors::autonomous_system_repository_error::AutonomousSystemRepositoryError;
use kernel::autonomous_system::domain::events::autonomous_system_created_event::AutonomousSystemCreatedEvent;
use kernel::autonomous_system::domain::repositories::autonomous_system_repository::AutonomousSystemRepository;
use kernel::autonomous_system::domain::value_objects::autonomous_system_id::AutonomousSystemId;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::autonomous_system::domain::value_objects::mothers::autonomous_system_value_mother::AutonomousSystemValueMother;
use crate::src::mocks::autonomous_system_repository_mock::AutonomousSystemRepositoryMock;
use crate::src::mocks::event_bus_mock::EventBusMock;

fn make_creator(
    repo: Arc<AutonomousSystemRepositoryMock>,
    bus: Arc<EventBusMock>,
) -> AutonomousSystemCreator {
    let repo: Arc<dyn AutonomousSystemRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    AutonomousSystemCreator::new(repo, bus)
}

#[tokio::test]
async fn it_saves_the_autonomous_system_under_the_id_derived_from_its_value() {
    let value = AutonomousSystemValueMother::create("15169");
    let expected_id = *AutonomousSystemId::from_value(&value).value();

    let repo = Arc::new(AutonomousSystemRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    creator.execute(value).await.unwrap();

    assert_eq!(repo.saved_ids(), vec![expected_id]);
}

#[tokio::test]
async fn it_publishes_a_created_event() {
    let repo = Arc::new(AutonomousSystemRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    creator
        .execute(AutonomousSystemValueMother::random())
        .await
        .unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![AutonomousSystemCreatedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_already_exists_error() {
    let repo = Arc::new(AutonomousSystemRepositoryMock::that_fails_with_already_exists());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator.execute(AutonomousSystemValueMother::random()).await;

    assert!(matches!(
        result,
        Err(AutonomousSystemRepositoryError::AlreadyExists)
    ));
}

#[tokio::test]
async fn it_does_not_publish_event_when_save_fails() {
    let repo = Arc::new(AutonomousSystemRepositoryMock::that_fails_with_already_exists());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let _ = creator.execute(AutonomousSystemValueMother::random()).await;

    assert!(bus.published_event_names().is_empty());
}

#[tokio::test]
async fn it_returns_unexpected_error_when_publishing_fails() {
    let repo = Arc::new(AutonomousSystemRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::that_fails());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator.execute(AutonomousSystemValueMother::random()).await;

    assert!(matches!(
        result,
        Err(AutonomousSystemRepositoryError::Unexpected(_))
    ));
}

#[tokio::test]
async fn it_returns_unexpected_error_when_save_fails() {
    let repo = Arc::new(AutonomousSystemRepositoryMock::that_fails_on_save(
        "storage error".to_string(),
    ));
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator.execute(AutonomousSystemValueMother::random()).await;

    assert!(matches!(
        result,
        Err(AutonomousSystemRepositoryError::Unexpected(_))
    ));
    assert!(bus.published_event_names().is_empty());
}
