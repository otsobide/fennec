use kernel::autonomous_system::infrastructure::persistence::in_memory::in_memory_autonomous_system_repository::InMemoryAutonomousSystemRepository;

use crate::src::autonomous_system::infrastructure::persistence::autonomous_system_repository_contract as contract;

#[tokio::test]
async fn it_finds_a_saved_autonomous_system() {
    contract::it_finds_a_saved_autonomous_system(&InMemoryAutonomousSystemRepository::new()).await;
}

#[tokio::test]
async fn it_rejects_registering_the_same_autonomous_system_twice() {
    contract::it_rejects_registering_the_same_autonomous_system_twice(
        &InMemoryAutonomousSystemRepository::new(),
    )
    .await;
}

#[tokio::test]
async fn it_returns_none_for_an_unknown_id() {
    contract::it_returns_none_for_an_unknown_id(&InMemoryAutonomousSystemRepository::new()).await;
}

#[tokio::test]
async fn it_deletes_a_saved_autonomous_system() {
    contract::it_deletes_a_saved_autonomous_system(&InMemoryAutonomousSystemRepository::new())
        .await;
}

#[tokio::test]
async fn it_returns_not_found_when_deleting_an_unknown_id() {
    contract::it_returns_not_found_when_deleting_an_unknown_id(
        &InMemoryAutonomousSystemRepository::new(),
    )
    .await;
}

#[tokio::test]
async fn it_accepts_a_deleted_autonomous_system_again() {
    contract::it_accepts_a_deleted_autonomous_system_again(
        &InMemoryAutonomousSystemRepository::new(),
    )
    .await;
}

#[tokio::test]
async fn it_keeps_the_description_of_a_saved_autonomous_system() {
    contract::it_keeps_the_description_of_a_saved_autonomous_system(
        &InMemoryAutonomousSystemRepository::new(),
    )
    .await;
}

#[tokio::test]
async fn it_replaces_the_description_on_update() {
    contract::it_replaces_the_description_on_update(&InMemoryAutonomousSystemRepository::new())
        .await;
}

#[tokio::test]
async fn it_returns_not_found_when_updating_an_unknown_autonomous_system() {
    contract::it_returns_not_found_when_updating_an_unknown_autonomous_system(
        &InMemoryAutonomousSystemRepository::new(),
    )
    .await;
}
