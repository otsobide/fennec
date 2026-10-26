use std::sync::Arc;

use kernel::ipv6_address::application::delete_ipv6_address::ipv6_address_deleter::Ipv6AddressDeleter;
use kernel::ipv6_address::domain::errors::ipv6_address_repository_error::Ipv6AddressRepositoryError;
use kernel::ipv6_address::domain::events::ipv6_address_deleted_event::Ipv6AddressDeletedEvent;
use kernel::ipv6_address::domain::repositories::ipv6_address_repository::Ipv6AddressRepository;
use shared_domain_events::domain::event_bus::EventBus;

use crate::src::ipv6_address::domain::entities::mothers::ipv6_address_mother::Ipv6AddressMother;
use crate::src::ipv6_address::domain::value_objects::mothers::ipv6_address_id_mother::Ipv6AddressIdMother;
use crate::src::mocks::event_bus_mock::EventBusMock;
use crate::src::mocks::ipv6_address_repository_mock::Ipv6AddressRepositoryMock;

fn make_deleter(
    repo: Arc<Ipv6AddressRepositoryMock>,
    bus: Arc<EventBusMock>,
) -> Ipv6AddressDeleter {
    let repo: Arc<dyn Ipv6AddressRepository> = repo;
    let bus: Arc<dyn EventBus> = bus;
    Ipv6AddressDeleter::new(repo, bus)
}

#[tokio::test]
async fn it_deletes_the_ipv6_address() {
    let ipv6_address = Ipv6AddressMother::random();
    let id = *ipv6_address.id();
    let repo = Arc::new(Ipv6AddressRepositoryMock::that_returns_ipv6_address(
        ipv6_address,
    ));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    deleter.execute(id).await.unwrap();

    assert_eq!(repo.delete_call_count(), 1);
}

#[tokio::test]
async fn it_publishes_a_deleted_event() {
    let ipv6_address = Ipv6AddressMother::random();
    let id = *ipv6_address.id();
    let repo = Arc::new(Ipv6AddressRepositoryMock::that_returns_ipv6_address(
        ipv6_address,
    ));
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    deleter.execute(id).await.unwrap();

    assert_eq!(
        bus.published_event_names(),
        vec![Ipv6AddressDeletedEvent::EVENT_NAME]
    );
}

#[tokio::test]
async fn it_returns_not_found_when_the_ipv6_address_does_not_exist() {
    let repo = Arc::new(Ipv6AddressRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let result = deleter.execute(Ipv6AddressIdMother::random()).await;

    assert!(matches!(result, Err(Ipv6AddressRepositoryError::NotFound)));
}

#[tokio::test]
async fn it_does_not_delete_or_publish_when_the_ipv6_address_does_not_exist() {
    let repo = Arc::new(Ipv6AddressRepositoryMock::that_finds_nothing());
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let _ = deleter.execute(Ipv6AddressIdMother::random()).await;

    assert_eq!(repo.delete_call_count(), 0);
    assert!(bus.published_event_names().is_empty());
}

#[tokio::test]
async fn it_does_not_publish_event_when_delete_fails() {
    let ipv6_address = Ipv6AddressMother::random();
    let id = *ipv6_address.id();
    let repo = Arc::new(
        Ipv6AddressRepositoryMock::that_returns_ipv6_address_but_delete_fails(
            ipv6_address,
            "storage error".to_string(),
        ),
    );
    let bus = Arc::new(EventBusMock::new());
    let deleter = make_deleter(repo.clone(), bus.clone());

    let result = deleter.execute(id).await;

    assert!(matches!(
        result,
        Err(Ipv6AddressRepositoryError::Unexpected(_))
    ));
    assert!(bus.published_event_names().is_empty());
}
