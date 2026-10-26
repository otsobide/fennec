use kernel::ipv4_network::infrastructure::persistence::in_memory::in_memory_ipv4_network_repository::InMemoryIpv4NetworkRepository;

use crate::src::ipv4_network::infrastructure::persistence::ipv4_network_repository_contract as contract;

#[tokio::test]
async fn it_finds_a_saved_ipv4_network() {
    contract::it_finds_a_saved_ipv4_network(&InMemoryIpv4NetworkRepository::new()).await;
}

#[tokio::test]
async fn it_rejects_registering_the_same_network_twice() {
    contract::it_rejects_registering_the_same_network_twice(&InMemoryIpv4NetworkRepository::new())
        .await;
}

#[tokio::test]
async fn it_returns_none_for_an_unknown_id() {
    contract::it_returns_none_for_an_unknown_id(&InMemoryIpv4NetworkRepository::new()).await;
}

#[tokio::test]
async fn it_deletes_a_saved_ipv4_network() {
    contract::it_deletes_a_saved_ipv4_network(&InMemoryIpv4NetworkRepository::new()).await;
}

#[tokio::test]
async fn it_returns_not_found_when_deleting_an_unknown_id() {
    contract::it_returns_not_found_when_deleting_an_unknown_id(
        &InMemoryIpv4NetworkRepository::new(),
    )
    .await;
}

#[tokio::test]
async fn it_accepts_a_deleted_network_again() {
    contract::it_accepts_a_deleted_network_again(&InMemoryIpv4NetworkRepository::new()).await;
}
