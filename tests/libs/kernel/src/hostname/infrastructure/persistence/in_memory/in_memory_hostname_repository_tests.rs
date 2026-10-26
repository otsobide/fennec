use kernel::hostname::infrastructure::persistence::in_memory::in_memory_hostname_repository::InMemoryHostnameRepository;

use crate::src::hostname::infrastructure::persistence::hostname_repository_contract as contract;

#[tokio::test]
async fn it_finds_a_saved_hostname() {
    contract::it_finds_a_saved_hostname(&InMemoryHostnameRepository::new()).await;
}

#[tokio::test]
async fn it_rejects_registering_the_same_hostname_twice() {
    contract::it_rejects_registering_the_same_hostname_twice(&InMemoryHostnameRepository::new())
        .await;
}

#[tokio::test]
async fn it_returns_none_for_an_unknown_id() {
    contract::it_returns_none_for_an_unknown_id(&InMemoryHostnameRepository::new()).await;
}

#[tokio::test]
async fn it_deletes_a_saved_hostname() {
    contract::it_deletes_a_saved_hostname(&InMemoryHostnameRepository::new()).await;
}

#[tokio::test]
async fn it_returns_not_found_when_deleting_an_unknown_id() {
    contract::it_returns_not_found_when_deleting_an_unknown_id(&InMemoryHostnameRepository::new())
        .await;
}

#[tokio::test]
async fn it_accepts_a_deleted_hostname_again() {
    contract::it_accepts_a_deleted_hostname_again(&InMemoryHostnameRepository::new()).await;
}
