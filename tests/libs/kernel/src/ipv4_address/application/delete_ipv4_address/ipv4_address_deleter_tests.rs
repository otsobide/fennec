use std::sync::Arc;

use kernel::ipv4_address::application::delete_ipv4_address::ipv4_address_deleter::Ipv4AddressDeleter;
use kernel::ipv4_address::domain::errors::ipv4_address_repository_error::Ipv4AddressRepositoryError;
use kernel::ipv4_address::domain::events::ipv4_address_deleted_event::Ipv4AddressDeletedEvent;
use kernel::ipv4_address::domain::repositories::ipv4_address_repository::Ipv4AddressRepository;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::ipv4_address::domain::entities::mothers::ipv4_address_mother::Ipv4AddressMother;
use crate::src::ipv4_address::domain::value_objects::mothers::ipv4_address_id_mother::Ipv4AddressIdMother;
use crate::src::mocks::event_bus_mock::EventBusMock;
use crate::src::mocks::ipv4_address_repository_mock::Ipv4AddressRepositoryMock;

fn make_deleter(
    repo: Arc<Ipv4AddressRepositoryMock>,
    bus: Arc<EventBusMock>,
) -> Ipv4AddressDeleter {
    let repo: Arc<dyn Ipv4AddressRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    Ipv4AddressDeleter::new(repo, bus)
}

#[tokio::test]
async fn it_deletes_the_ipv4_address() {
    let ipv4_address = Ipv4AddressMother::random();
    let id = *ipv4_address.id();
    let repo = Arc::new(Ipv4AddressRepositoryMock::that_returns_ipv4_address(
        ipv4_address,
    ));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    deleter.execute(id).await.unwrap();

    assert_eq!(repo.delete_call_count(), 1);
}

#[tokio::test]
async fn it_publishes_a_deleted_event() {
    let ipv4_address = Ipv4AddressMother::random();
    let id = *ipv4_address.id();
    let repo = Arc::new(Ipv4AddressRepositoryMock::that_returns_ipv4_address(
        ipv4_address,
    ));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    deleter.execute(id).await.unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![Ipv4AddressDeletedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_not_found_when_the_ipv4_address_does_not_exist() {
    let repo = Arc::new(Ipv4AddressRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let result = deleter.execute(Ipv4AddressIdMother::random()).await;

    assert!(matches!(result, Err(Ipv4AddressRepositoryError::NotFound)));
}

#[tokio::test]
async fn it_does_not_delete_or_publish_when_the_ipv4_address_does_not_exist() {
    let repo = Arc::new(Ipv4AddressRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let _ = deleter.execute(Ipv4AddressIdMother::random()).await;

    assert_eq!(repo.delete_call_count(), 0);
    assert!(bus.published_event_names().is_empty());
}

#[tokio::test]
async fn it_does_not_publish_event_when_delete_fails() {
    let ipv4_address = Ipv4AddressMother::random();
    let id = *ipv4_address.id();
    let repo = Arc::new(
        Ipv4AddressRepositoryMock::that_returns_ipv4_address_but_delete_fails(
            ipv4_address,
            "storage error".to_string(),
        ),
    );
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let result = deleter.execute(id).await;

    assert!(matches!(
        result,
        Err(Ipv4AddressRepositoryError::Unexpected(_))
    ));
    assert!(bus.published_event_names().is_empty());
}
