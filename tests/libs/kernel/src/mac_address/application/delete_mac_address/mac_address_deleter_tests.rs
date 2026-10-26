use std::sync::Arc;

use kernel::mac_address::application::delete_mac_address::mac_address_deleter::MacAddressDeleter;
use kernel::mac_address::domain::errors::mac_address_repository_error::MacAddressRepositoryError;
use kernel::mac_address::domain::events::mac_address_deleted_event::MacAddressDeletedEvent;
use kernel::mac_address::domain::repositories::mac_address_repository::MacAddressRepository;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::mac_address::domain::entities::mothers::mac_address_mother::MacAddressMother;
use crate::src::mac_address::domain::value_objects::mothers::mac_address_id_mother::MacAddressIdMother;
use crate::src::mocks::event_bus_mock::EventBusMock;
use crate::src::mocks::mac_address_repository_mock::MacAddressRepositoryMock;

fn make_deleter(repo: Arc<MacAddressRepositoryMock>, bus: Arc<EventBusMock>) -> MacAddressDeleter {
    let repo: Arc<dyn MacAddressRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    MacAddressDeleter::new(repo, bus)
}

#[tokio::test]
async fn it_deletes_the_mac_address() {
    let mac_address = MacAddressMother::random();
    let id = *mac_address.id();
    let repo = Arc::new(MacAddressRepositoryMock::that_returns_mac_address(
        mac_address,
    ));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    deleter.execute(id).await.unwrap();

    assert_eq!(repo.delete_call_count(), 1);
}

#[tokio::test]
async fn it_publishes_a_deleted_event() {
    let mac_address = MacAddressMother::random();
    let id = *mac_address.id();
    let repo = Arc::new(MacAddressRepositoryMock::that_returns_mac_address(
        mac_address,
    ));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    deleter.execute(id).await.unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![MacAddressDeletedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_not_found_when_the_mac_address_does_not_exist() {
    let repo = Arc::new(MacAddressRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let result = deleter.execute(MacAddressIdMother::random()).await;

    assert!(matches!(result, Err(MacAddressRepositoryError::NotFound)));
}

#[tokio::test]
async fn it_does_not_delete_or_publish_when_the_mac_address_does_not_exist() {
    let repo = Arc::new(MacAddressRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let _ = deleter.execute(MacAddressIdMother::random()).await;

    assert_eq!(repo.delete_call_count(), 0);
    assert!(bus.published_event_names().is_empty());
}

#[tokio::test]
async fn it_does_not_publish_event_when_delete_fails() {
    let mac_address = MacAddressMother::random();
    let id = *mac_address.id();
    let repo = Arc::new(
        MacAddressRepositoryMock::that_returns_mac_address_but_delete_fails(
            mac_address,
            "storage error".to_string(),
        ),
    );
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let result = deleter.execute(id).await;

    assert!(matches!(
        result,
        Err(MacAddressRepositoryError::Unexpected(_))
    ));
    assert!(bus.published_event_names().is_empty());
}
