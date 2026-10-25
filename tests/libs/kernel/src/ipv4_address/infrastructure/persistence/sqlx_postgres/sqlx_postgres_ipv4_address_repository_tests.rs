use kernel::ipv4_address::domain::errors::ipv4_address_repository_error::Ipv4AddressRepositoryError;
use kernel::ipv4_address::domain::repositories::ipv4_address_repository::Ipv4AddressRepository;
use kernel::ipv4_address::infrastructure::persistence::sqlx_postgres::sqlx_postgres_ipv4_address_repository::SqlxPostgresIpv4AddressRepository;
use sqlx::PgPool;

use crate::src::ipv4_address::domain::entities::mothers::ipv4_address_mother::Ipv4AddressMother;
use crate::src::ipv4_address::domain::value_objects::mothers::ipv4_address_id_mother::Ipv4AddressIdMother;
use crate::src::ipv4_address::infrastructure::persistence::ipv4_address_repository_contract as contract;

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_finds_a_saved_ipv4_address(pool: PgPool) {
    contract::it_finds_a_saved_ipv4_address(&SqlxPostgresIpv4AddressRepository::new(pool)).await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_rejects_registering_the_same_address_twice(pool: PgPool) {
    contract::it_rejects_registering_the_same_address_twice(
        &SqlxPostgresIpv4AddressRepository::new(pool),
    )
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_returns_none_for_an_unknown_id(pool: PgPool) {
    contract::it_returns_none_for_an_unknown_id(&SqlxPostgresIpv4AddressRepository::new(pool))
        .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_deletes_a_saved_ipv4_address(pool: PgPool) {
    contract::it_deletes_a_saved_ipv4_address(&SqlxPostgresIpv4AddressRepository::new(pool)).await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_returns_not_found_when_deleting_an_unknown_id(pool: PgPool) {
    contract::it_returns_not_found_when_deleting_an_unknown_id(
        &SqlxPostgresIpv4AddressRepository::new(pool),
    )
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_accepts_a_deleted_address_again(pool: PgPool) {
    contract::it_accepts_a_deleted_address_again(&SqlxPostgresIpv4AddressRepository::new(pool))
        .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_reports_a_row_whose_id_does_not_match_its_value(pool: PgPool) {
    let tampered = Ipv4AddressIdMother::create("89.19.209.154");
    sqlx::query("INSERT INTO kernel.ipv4_address (id, value, created_at) VALUES ($1, '198.51.100.7', now())")
        .bind(*tampered.value())
        .execute(&pool)
        .await
        .unwrap();

    let result = SqlxPostgresIpv4AddressRepository::new(pool)
        .find_by_id(&tampered)
        .await;

    assert!(matches!(
        result,
        Err(Ipv4AddressRepositoryError::Unexpected(_))
    ));
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_stores_the_address_as_a_host_inet(pool: PgPool) {
    let repository = SqlxPostgresIpv4AddressRepository::new(pool.clone());
    repository
        .save(&Ipv4AddressMother::create("198.51.100.7/32"))
        .await
        .unwrap();

    let stored: String = sqlx::query_scalar("SELECT value::text FROM kernel.ipv4_address")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(stored, "198.51.100.7/32");
}
