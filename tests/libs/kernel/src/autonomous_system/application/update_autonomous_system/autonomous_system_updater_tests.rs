use std::sync::Arc;

use kernel::autonomous_system::application::update_autonomous_system::autonomous_system_updater::AutonomousSystemUpdater;
use kernel::autonomous_system::domain::errors::autonomous_system_repository_error::AutonomousSystemRepositoryError;
use kernel::autonomous_system::domain::events::autonomous_system_updated_event::AutonomousSystemUpdatedEvent;
use kernel::autonomous_system::domain::repositories::autonomous_system_repository::AutonomousSystemRepository;
use kernel::autonomous_system::domain::value_objects::autonomous_system_rir::AutonomousSystemRir;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::autonomous_system::domain::entities::mothers::autonomous_system_mother::AutonomousSystemMother;
use crate::src::autonomous_system::domain::value_objects::mothers::autonomous_system_id_mother::AutonomousSystemIdMother;
use crate::src::autonomous_system::domain::value_objects::mothers::autonomous_system_name_mother::AutonomousSystemNameMother;
use crate::src::mocks::autonomous_system_repository_mock::AutonomousSystemRepositoryMock;
use crate::src::mocks::event_bus_mock::EventBusMock;

fn make_updater(
    repo: Arc<AutonomousSystemRepositoryMock>,
    bus: Arc<EventBusMock>,
) -> AutonomousSystemUpdater {
    let repo: Arc<dyn AutonomousSystemRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    AutonomousSystemUpdater::new(repo, bus)
}

#[tokio::test]
async fn it_stores_the_new_description_and_keeps_the_identity() {
    let registered = AutonomousSystemMother::create("15169");
    let id = *registered.id();
    let repo = Arc::new(
        AutonomousSystemRepositoryMock::that_returns_autonomous_system(registered.clone()),
    );
    let bus = Arc::new(EventBusMock::new());
    let updater = make_updater(repo.clone(), bus.clone());
    let name = AutonomousSystemNameMother::create("GOOGLE");

    updater
        .execute(id, Some(name.clone()), Some(AutonomousSystemRir::Arin))
        .await
        .unwrap();

    let updated = repo.updated();
    assert_eq!(updated.len(), 1);
    assert_eq!(updated[0].id(), registered.id());
    assert_eq!(updated[0].value(), registered.value());
    assert_eq!(updated[0].created_at(), registered.created_at());
    assert_eq!(updated[0].name(), Some(&name));
    assert_eq!(updated[0].rir(), Some(&AutonomousSystemRir::Arin));
    assert!(updated[0].updated_at().unix_micros() >= registered.updated_at().unix_micros());
}

#[tokio::test]
async fn it_publishes_an_updated_event() {
    let registered = AutonomousSystemMother::random();
    let id = *registered.id();
    let repo = Arc::new(AutonomousSystemRepositoryMock::that_returns_autonomous_system(registered));
    let bus = Arc::new(EventBusMock::new());
    let updater = make_updater(repo.clone(), bus.clone());

    updater
        .execute(id, None, Some(AutonomousSystemRir::RipeNcc))
        .await
        .unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![AutonomousSystemUpdatedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_not_found_when_the_autonomous_system_does_not_exist() {
    let repo = Arc::new(AutonomousSystemRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let updater = make_updater(repo.clone(), bus.clone());

    let result = updater
        .execute(AutonomousSystemIdMother::random(), None, None)
        .await;

    assert!(matches!(
        result,
        Err(AutonomousSystemRepositoryError::NotFound)
    ));
}

#[tokio::test]
async fn it_does_not_update_or_publish_when_the_autonomous_system_does_not_exist() {
    let repo = Arc::new(AutonomousSystemRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let updater = make_updater(repo.clone(), bus.clone());

    let _ = updater
        .execute(AutonomousSystemIdMother::random(), None, None)
        .await;

    assert!(repo.updated().is_empty());
    assert!(bus.published_event_names().is_empty());
}

#[tokio::test]
async fn it_does_not_publish_event_when_update_fails() {
    let registered = AutonomousSystemMother::random();
    let id = *registered.id();
    let repo = Arc::new(
        AutonomousSystemRepositoryMock::that_returns_autonomous_system_but_update_fails(
            registered,
            "storage error".to_string(),
        ),
    );
    let bus = Arc::new(EventBusMock::new());
    let updater = make_updater(repo.clone(), bus.clone());

    let result = updater.execute(id, None, None).await;

    assert!(matches!(
        result,
        Err(AutonomousSystemRepositoryError::Unexpected(_))
    ));
    assert!(bus.published_event_names().is_empty());
}

#[tokio::test]
async fn it_returns_unexpected_error_when_publishing_fails() {
    let registered = AutonomousSystemMother::random();
    let id = *registered.id();
    let repo = Arc::new(AutonomousSystemRepositoryMock::that_returns_autonomous_system(registered));
    let bus = Arc::new(EventBusMock::that_fails());
    let updater = make_updater(repo.clone(), bus.clone());

    let result = updater.execute(id, None, None).await;

    assert!(matches!(
        result,
        Err(AutonomousSystemRepositoryError::Unexpected(_))
    ));
}
