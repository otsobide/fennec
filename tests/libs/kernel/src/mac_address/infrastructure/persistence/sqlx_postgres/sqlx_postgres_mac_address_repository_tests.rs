use kernel::mac_address::domain::errors::mac_address_repository_error::MacAddressRepositoryError;
use kernel::mac_address::domain::repositories::mac_address_repository::MacAddressRepository;
use kernel::mac_address::infrastructure::persistence::sqlx_postgres::sqlx_postgres_mac_address_repository::SqlxPostgresMacAddressRepository;
use sqlx::PgPool;

use crate::src::mac_address::domain::entities::mothers::mac_address_mother::MacAddressMother;
use crate::src::mac_address::domain::value_objects::mothers::mac_address_id_mother::MacAddressIdMother;
use crate::src::mac_address::infrastructure::persistence::mac_address_repository_contract as contract;

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_finds_a_saved_mac_address(pool: PgPool) {
    contract::it_finds_a_saved_mac_address(&SqlxPostgresMacAddressRepository::new(pool)).await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_rejects_registering_the_same_address_twice(pool: PgPool) {
    contract::it_rejects_registering_the_same_address_twice(
        &SqlxPostgresMacAddressRepository::new(pool),
    )
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_returns_none_for_an_unknown_id(pool: PgPool) {
    contract::it_returns_none_for_an_unknown_id(&SqlxPostgresMacAddressRepository::new(pool)).await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_deletes_a_saved_mac_address(pool: PgPool) {
    contract::it_deletes_a_saved_mac_address(&SqlxPostgresMacAddressRepository::new(pool)).await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_returns_not_found_when_deleting_an_unknown_id(pool: PgPool) {
    contract::it_returns_not_found_when_deleting_an_unknown_id(
        &SqlxPostgresMacAddressRepository::new(pool),
    )
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_accepts_a_deleted_address_again(pool: PgPool) {
    contract::it_accepts_a_deleted_address_again(&SqlxPostgresMacAddressRepository::new(pool))
        .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_reports_a_row_whose_id_does_not_match_its_value(pool: PgPool) {
    let tampered = MacAddressIdMother::create("d2:fb:49:24:37:18");
    sqlx::query(
        "INSERT INTO kernel.mac_address (id, value, created_at) VALUES ($1, $2::macaddr, now())",
    )
    .bind(*tampered.value())
    .bind("02:00:5e:10:00:00")
    .execute(&pool)
    .await
    .unwrap();

    let result = SqlxPostgresMacAddressRepository::new(pool)
        .find_by_id(&tampered)
        .await;

    assert!(matches!(
        result,
        Err(MacAddressRepositoryError::Unexpected(_))
    ));
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_stores_the_canonical_value(pool: PgPool) {
    let repository = SqlxPostgresMacAddressRepository::new(pool.clone());
    repository
        .save(&MacAddressMother::create("02005E100000"))
        .await
        .unwrap();

    let stored: String = sqlx::query_scalar("SELECT value::text FROM kernel.mac_address")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(stored, "02:00:5e:10:00:00");
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_lets_the_column_type_canonicalise_a_raw_value(pool: PgPool) {
    sqlx::query(
        "INSERT INTO kernel.mac_address (id, value, created_at) VALUES ($1, $2::macaddr, now())",
    )
    .bind(*MacAddressIdMother::create("d2:fb:49:24:37:18").value())
    .bind("D2-FB-49-24-37-18")
    .execute(&pool)
    .await
    .unwrap();

    let stored: String = sqlx::query_scalar("SELECT value::text FROM kernel.mac_address")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(stored, "d2:fb:49:24:37:18");
}
