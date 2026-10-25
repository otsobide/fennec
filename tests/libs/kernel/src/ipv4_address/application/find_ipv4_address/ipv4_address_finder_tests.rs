use std::sync::Arc;

use kernel::ipv4_address::application::find_ipv4_address::ipv4_address_finder::Ipv4AddressFinder;
use kernel::ipv4_address::domain::errors::ipv4_address_repository_error::Ipv4AddressRepositoryError;
use kernel::ipv4_address::domain::repositories::ipv4_address_repository::Ipv4AddressRepository;

use crate::src::ipv4_address::domain::entities::mothers::ipv4_address_mother::Ipv4AddressMother;
use crate::src::ipv4_address::domain::value_objects::mothers::ipv4_address_id_mother::Ipv4AddressIdMother;
use crate::src::mocks::ipv4_address_repository_mock::Ipv4AddressRepositoryMock;

fn make_finder(repo: Arc<Ipv4AddressRepositoryMock>) -> Ipv4AddressFinder {
    let repo: Arc<dyn Ipv4AddressRepository> = repo;
    Ipv4AddressFinder::new(repo)
}

#[tokio::test]
async fn it_returns_the_ipv4_address_when_it_exists() {
    let ipv4_address = Ipv4AddressMother::random();
    let expected = ipv4_address.clone();
    let repo = Arc::new(Ipv4AddressRepositoryMock::that_returns_ipv4_address(
        ipv4_address,
    ));
    let finder = make_finder(repo);

    let result = finder.execute(*expected.id()).await.unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn it_returns_not_found_when_the_ipv4_address_does_not_exist() {
    let repo = Arc::new(Ipv4AddressRepositoryMock::that_finds_nothing());
    let finder = make_finder(repo);

    let result = finder.execute(Ipv4AddressIdMother::random()).await;

    assert!(matches!(result, Err(Ipv4AddressRepositoryError::NotFound)));
}

#[tokio::test]
async fn it_returns_error_on_storage_failure() {
    let repo = Arc::new(Ipv4AddressRepositoryMock::that_fails_on_find(
        "storage error".to_string(),
    ));
    let finder = make_finder(repo);

    let result = finder.execute(Ipv4AddressIdMother::random()).await;

    assert!(matches!(
        result,
        Err(Ipv4AddressRepositoryError::Unexpected(_))
    ));
}
