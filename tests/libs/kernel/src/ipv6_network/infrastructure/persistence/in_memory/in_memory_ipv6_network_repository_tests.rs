use kernel::ipv6_network::infrastructure::persistence::in_memory::in_memory_ipv6_network_repository::InMemoryIpv6NetworkRepository;

use crate::src::ipv6_network::infrastructure::persistence::ipv6_network_repository_contract as contract;

#[tokio::test]
async fn it_finds_a_saved_ipv6_network() {
    contract::it_finds_a_saved_ipv6_network(&InMemoryIpv6NetworkRepository::new()).await;
}

#[tokio::test]
async fn it_rejects_registering_the_same_network_twice() {
    contract::it_rejects_registering_the_same_network_twice(&InMemoryIpv6NetworkRepository::new())
        .await;
}

#[tokio::test]
async fn it_returns_none_for_an_unknown_id() {
    contract::it_returns_none_for_an_unknown_id(&InMemoryIpv6NetworkRepository::new()).await;
}

#[tokio::test]
async fn it_deletes_a_saved_ipv6_network() {
    contract::it_deletes_a_saved_ipv6_network(&InMemoryIpv6NetworkRepository::new()).await;
}

#[tokio::test]
async fn it_returns_not_found_when_deleting_an_unknown_id() {
    contract::it_returns_not_found_when_deleting_an_unknown_id(
        &InMemoryIpv6NetworkRepository::new(),
    )
    .await;
}

#[tokio::test]
async fn it_accepts_a_deleted_network_again() {
    contract::it_accepts_a_deleted_network_again(&InMemoryIpv6NetworkRepository::new()).await;
}
