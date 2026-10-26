use std::sync::Arc;

use kernel::autonomous_system::application::delete_autonomous_system::autonomous_system_deleter::AutonomousSystemDeleter;
use kernel::autonomous_system::domain::errors::autonomous_system_repository_error::AutonomousSystemRepositoryError;
use kernel::autonomous_system::domain::events::autonomous_system_deleted_event::AutonomousSystemDeletedEvent;
use kernel::autonomous_system::domain::repositories::autonomous_system_repository::AutonomousSystemRepository;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::autonomous_system::domain::entities::mothers::autonomous_system_mother::AutonomousSystemMother;
use crate::src::autonomous_system::domain::value_objects::mothers::autonomous_system_id_mother::AutonomousSystemIdMother;
use crate::src::mocks::autonomous_system_repository_mock::AutonomousSystemRepositoryMock;
use crate::src::mocks::event_bus_mock::EventBusMock;

fn make_deleter(
    repo: Arc<AutonomousSystemRepositoryMock>,
    bus: Arc<EventBusMock>,
) -> AutonomousSystemDeleter {
    let repo: Arc<dyn AutonomousSystemRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    AutonomousSystemDeleter::new(repo, bus)
}

#[tokio::test]
async fn it_deletes_the_autonomous_system() {
    let autonomous_system = AutonomousSystemMother::random();
    let id = *autonomous_system.id();
    let repo =
        Arc::new(AutonomousSystemRepositoryMock::that_returns_autonomous_system(autonomous_system));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    deleter.execute(id).await.unwrap();

    assert_eq!(repo.delete_call_count(), 1);
}

#[tokio::test]
async fn it_publishes_a_deleted_event() {
    let autonomous_system = AutonomousSystemMother::random();
    let id = *autonomous_system.id();
    let repo =
        Arc::new(AutonomousSystemRepositoryMock::that_returns_autonomous_system(autonomous_system));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    deleter.execute(id).await.unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![AutonomousSystemDeletedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_not_found_when_the_autonomous_system_does_not_exist() {
    let repo = Arc::new(AutonomousSystemRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let result = deleter.execute(AutonomousSystemIdMother::random()).await;

    assert!(matches!(
        result,
        Err(AutonomousSystemRepositoryError::NotFound)
    ));
}

#[tokio::test]
async fn it_does_not_delete_or_publish_when_the_autonomous_system_does_not_exist() {
    let repo = Arc::new(AutonomousSystemRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let _ = deleter.execute(AutonomousSystemIdMother::random()).await;

    assert_eq!(repo.delete_call_count(), 0);
    assert!(bus.published_event_names().is_empty());
}

#[tokio::test]
async fn it_does_not_publish_event_when_delete_fails() {
    let autonomous_system = AutonomousSystemMother::random();
    let id = *autonomous_system.id();
    let repo = Arc::new(
        AutonomousSystemRepositoryMock::that_returns_autonomous_system_but_delete_fails(
            autonomous_system,
            "storage error".to_string(),
        ),
    );
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let result = deleter.execute(id).await;

    assert!(matches!(
        result,
        Err(AutonomousSystemRepositoryError::Unexpected(_))
    ));
    assert!(bus.published_event_names().is_empty());
}
