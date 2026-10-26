use std::sync::Arc;

use kernel::mac_address::application::create_mac_address::mac_address_creator::MacAddressCreator;
use kernel::mac_address::domain::errors::mac_address_repository_error::MacAddressRepositoryError;
use kernel::mac_address::domain::events::mac_address_created_event::MacAddressCreatedEvent;
use kernel::mac_address::domain::repositories::mac_address_repository::MacAddressRepository;
use kernel::mac_address::domain::value_objects::mac_address_id::MacAddressId;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::mac_address::domain::value_objects::mothers::mac_address_value_mother::MacAddressValueMother;
use crate::src::mocks::event_bus_mock::EventBusMock;
use crate::src::mocks::mac_address_repository_mock::MacAddressRepositoryMock;

fn make_creator(repo: Arc<MacAddressRepositoryMock>, bus: Arc<EventBusMock>) -> MacAddressCreator {
    let repo: Arc<dyn MacAddressRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    MacAddressCreator::new(repo, bus)
}

#[tokio::test]
async fn it_saves_the_mac_address_under_the_id_derived_from_its_value() {
    let value = MacAddressValueMother::create("d2:fb:49:24:37:18");
    let expected_id = *MacAddressId::from_value(&value).value();

    let repo = Arc::new(MacAddressRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    creator.execute(value).await.unwrap();

    assert_eq!(repo.saved_ids(), vec![expected_id]);
}

#[tokio::test]
async fn it_publishes_a_created_event() {
    let repo = Arc::new(MacAddressRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    creator
        .execute(MacAddressValueMother::random())
        .await
        .unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![MacAddressCreatedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_already_exists_error() {
    let repo = Arc::new(MacAddressRepositoryMock::that_fails_with_already_exists());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator.execute(MacAddressValueMother::random()).await;

    assert!(matches!(
        result,
        Err(MacAddressRepositoryError::AlreadyExists)
    ));
}

#[tokio::test]
async fn it_does_not_publish_event_when_save_fails() {
    let repo = Arc::new(MacAddressRepositoryMock::that_fails_with_already_exists());
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let _ = creator.execute(MacAddressValueMother::random()).await;

    assert!(bus.published_event_names().is_empty());
}

#[tokio::test]
async fn it_returns_unexpected_error_when_publishing_fails() {
    let repo = Arc::new(MacAddressRepositoryMock::that_succeeds());
    let bus = Arc::new(EventBusMock::that_fails());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator.execute(MacAddressValueMother::random()).await;

    assert!(matches!(
        result,
        Err(MacAddressRepositoryError::Unexpected(_))
    ));
}

#[tokio::test]
async fn it_returns_unexpected_error_when_save_fails() {
    let repo = Arc::new(MacAddressRepositoryMock::that_fails_on_save(
        "storage error".to_string(),
    ));
    let bus = Arc::new(EventBusMock::new());
    let creator = make_creator(repo.clone(), bus.clone());

    let result = creator.execute(MacAddressValueMother::random()).await;

    assert!(matches!(
        result,
        Err(MacAddressRepositoryError::Unexpected(_))
    ));
    assert!(bus.published_event_names().is_empty());
}
