use kernel::url_source::infrastructure::persistence::in_memory::in_memory_url_source_repository::InMemoryUrlSourceRepository;

use crate::src::url_source::infrastructure::persistence::url_source_repository_contract as contract;

#[tokio::test]
async fn it_finds_a_saved_url_source() {
    contract::it_finds_a_saved_url_source(&InMemoryUrlSourceRepository::new()).await;
}

#[tokio::test]
async fn it_rejects_saving_the_same_id_twice() {
    contract::it_rejects_saving_the_same_id_twice(&InMemoryUrlSourceRepository::new()).await;
}

#[tokio::test]
async fn it_returns_none_for_an_unknown_id() {
    contract::it_returns_none_for_an_unknown_id(&InMemoryUrlSourceRepository::new()).await;
}

#[tokio::test]
async fn it_updates_a_saved_url_source() {
    contract::it_updates_a_saved_url_source(&InMemoryUrlSourceRepository::new()).await;
}

#[tokio::test]
async fn it_returns_not_found_when_updating_an_unknown_url_source() {
    contract::it_returns_not_found_when_updating_an_unknown_url_source(
        &InMemoryUrlSourceRepository::new(),
    )
    .await;
}

#[tokio::test]
async fn it_deletes_a_saved_url_source() {
    contract::it_deletes_a_saved_url_source(&InMemoryUrlSourceRepository::new()).await;
}

#[tokio::test]
async fn it_returns_not_found_when_deleting_an_unknown_id() {
    contract::it_returns_not_found_when_deleting_an_unknown_id(&InMemoryUrlSourceRepository::new())
        .await;
}
