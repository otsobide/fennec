use std::sync::Arc;

use kernel::ipv6_address::application::find_ipv6_address::ipv6_address_finder::Ipv6AddressFinder;
use kernel::ipv6_address::domain::errors::ipv6_address_repository_error::Ipv6AddressRepositoryError;
use kernel::ipv6_address::domain::repositories::ipv6_address_repository::Ipv6AddressRepository;

use crate::src::ipv6_address::domain::entities::mothers::ipv6_address_mother::Ipv6AddressMother;
use crate::src::ipv6_address::domain::value_objects::mothers::ipv6_address_id_mother::Ipv6AddressIdMother;
use crate::src::mocks::ipv6_address_repository_mock::Ipv6AddressRepositoryMock;

fn make_finder(repo: Arc<Ipv6AddressRepositoryMock>) -> Ipv6AddressFinder {
    let repo: Arc<dyn Ipv6AddressRepository> = repo;
    Ipv6AddressFinder::new(repo)
}

#[tokio::test]
async fn it_returns_the_ipv6_address_when_it_exists() {
    let ipv6_address = Ipv6AddressMother::random();
    let expected = ipv6_address.clone();
    let repo = Arc::new(Ipv6AddressRepositoryMock::that_returns_ipv6_address(
        ipv6_address,
    ));
    let finder = make_finder(repo);

    let result = finder.execute(*expected.id()).await.unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn it_returns_not_found_when_the_ipv6_address_does_not_exist() {
    let repo = Arc::new(Ipv6AddressRepositoryMock::that_finds_nothing());
    let finder = make_finder(repo);

    let result = finder.execute(Ipv6AddressIdMother::random()).await;

    assert!(matches!(result, Err(Ipv6AddressRepositoryError::NotFound)));
}

#[tokio::test]
async fn it_returns_error_on_storage_failure() {
    let repo = Arc::new(Ipv6AddressRepositoryMock::that_fails_on_find(
        "storage error".to_string(),
    ));
    let finder = make_finder(repo);

    let result = finder.execute(Ipv6AddressIdMother::random()).await;

    assert!(matches!(
        result,
        Err(Ipv6AddressRepositoryError::Unexpected(_))
    ));
}
