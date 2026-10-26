use std::sync::Arc;

use kernel::mac_address::application::find_mac_address::mac_address_finder::MacAddressFinder;
use kernel::mac_address::domain::errors::mac_address_repository_error::MacAddressRepositoryError;
use kernel::mac_address::domain::repositories::mac_address_repository::MacAddressRepository;

use crate::src::mac_address::domain::entities::mothers::mac_address_mother::MacAddressMother;
use crate::src::mac_address::domain::value_objects::mothers::mac_address_id_mother::MacAddressIdMother;
use crate::src::mocks::mac_address_repository_mock::MacAddressRepositoryMock;

fn make_finder(repo: Arc<MacAddressRepositoryMock>) -> MacAddressFinder {
    let repo: Arc<dyn MacAddressRepository> = repo;
    MacAddressFinder::new(repo)
}

#[tokio::test]
async fn it_returns_the_mac_address_when_it_exists() {
    let mac_address = MacAddressMother::random();
    let expected = mac_address.clone();
    let repo = Arc::new(MacAddressRepositoryMock::that_returns_mac_address(
        mac_address,
    ));
    let finder = make_finder(repo);

    let result = finder.execute(*expected.id()).await.unwrap();

    assert_eq!(result, expected);
}

#[tokio::test]
async fn it_returns_not_found_when_the_mac_address_does_not_exist() {
    let repo = Arc::new(MacAddressRepositoryMock::that_finds_nothing());
    let finder = make_finder(repo);

    let result = finder.execute(MacAddressIdMother::random()).await;

    assert!(matches!(result, Err(MacAddressRepositoryError::NotFound)));
}

#[tokio::test]
async fn it_returns_error_on_storage_failure() {
    let repo = Arc::new(MacAddressRepositoryMock::that_fails_on_find(
        "storage error".to_string(),
    ));
    let finder = make_finder(repo);

    let result = finder.execute(MacAddressIdMother::random()).await;

    assert!(matches!(
        result,
        Err(MacAddressRepositoryError::Unexpected(_))
    ));
}
