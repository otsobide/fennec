use kernel::ipv6_address::infrastructure::persistence::in_memory::in_memory_ipv6_address_repository::InMemoryIpv6AddressRepository;

use crate::src::ipv6_address::infrastructure::persistence::ipv6_address_repository_contract as contract;

#[tokio::test]
async fn it_finds_a_saved_ipv6_address() {
    contract::it_finds_a_saved_ipv6_address(&InMemoryIpv6AddressRepository::new()).await;
}

#[tokio::test]
async fn it_rejects_registering_the_same_address_twice() {
    contract::it_rejects_registering_the_same_address_twice(&InMemoryIpv6AddressRepository::new())
        .await;
}

#[tokio::test]
async fn it_returns_none_for_an_unknown_id() {
    contract::it_returns_none_for_an_unknown_id(&InMemoryIpv6AddressRepository::new()).await;
}

#[tokio::test]
async fn it_deletes_a_saved_ipv6_address() {
    contract::it_deletes_a_saved_ipv6_address(&InMemoryIpv6AddressRepository::new()).await;
}

#[tokio::test]
async fn it_returns_not_found_when_deleting_an_unknown_id() {
    contract::it_returns_not_found_when_deleting_an_unknown_id(
        &InMemoryIpv6AddressRepository::new(),
    )
    .await;
}

#[tokio::test]
async fn it_accepts_a_deleted_address_again() {
    contract::it_accepts_a_deleted_address_again(&InMemoryIpv6AddressRepository::new()).await;
}
