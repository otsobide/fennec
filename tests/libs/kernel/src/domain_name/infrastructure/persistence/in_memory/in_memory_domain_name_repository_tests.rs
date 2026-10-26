use kernel::domain_name::infrastructure::persistence::in_memory::in_memory_domain_name_repository::InMemoryDomainNameRepository;

use crate::src::domain_name::infrastructure::persistence::domain_name_repository_contract as contract;

#[tokio::test]
async fn it_finds_a_saved_domain_name() {
    contract::it_finds_a_saved_domain_name(&InMemoryDomainNameRepository::new()).await;
}

#[tokio::test]
async fn it_rejects_registering_the_same_name_twice() {
    contract::it_rejects_registering_the_same_name_twice(&InMemoryDomainNameRepository::new())
        .await;
}

#[tokio::test]
async fn it_returns_none_for_an_unknown_id() {
    contract::it_returns_none_for_an_unknown_id(&InMemoryDomainNameRepository::new()).await;
}

#[tokio::test]
async fn it_deletes_a_saved_domain_name() {
    contract::it_deletes_a_saved_domain_name(&InMemoryDomainNameRepository::new()).await;
}

#[tokio::test]
async fn it_returns_not_found_when_deleting_an_unknown_id() {
    contract::it_returns_not_found_when_deleting_an_unknown_id(
        &InMemoryDomainNameRepository::new(),
    )
    .await;
}

#[tokio::test]
async fn it_accepts_a_deleted_name_again() {
    contract::it_accepts_a_deleted_name_again(&InMemoryDomainNameRepository::new()).await;
}
