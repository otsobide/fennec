use kernel::source::infrastructure::persistence::sqlx_postgres::sqlx_postgres_source_repository::SqlxPostgresSourceRepository;
use sqlx::PgPool;

use crate::src::source::infrastructure::persistence::source_repository_contract as contract;

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_finds_a_saved_source(pool: PgPool) {
    contract::it_finds_a_saved_source(&SqlxPostgresSourceRepository::new(pool)).await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_rejects_saving_the_same_id_twice(pool: PgPool) {
    contract::it_rejects_saving_the_same_id_twice(&SqlxPostgresSourceRepository::new(pool)).await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_returns_none_for_an_unknown_id(pool: PgPool) {
    contract::it_returns_none_for_an_unknown_id(&SqlxPostgresSourceRepository::new(pool)).await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_updates_a_saved_source(pool: PgPool) {
    contract::it_updates_a_saved_source(&SqlxPostgresSourceRepository::new(pool)).await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_returns_not_found_when_updating_an_unknown_source(pool: PgPool) {
    contract::it_returns_not_found_when_updating_an_unknown_source(
        &SqlxPostgresSourceRepository::new(pool),
    )
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_deletes_a_saved_source(pool: PgPool) {
    contract::it_deletes_a_saved_source(&SqlxPostgresSourceRepository::new(pool)).await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_returns_not_found_when_deleting_an_unknown_id(pool: PgPool) {
    contract::it_returns_not_found_when_deleting_an_unknown_id(&SqlxPostgresSourceRepository::new(
        pool,
    ))
    .await;
}
