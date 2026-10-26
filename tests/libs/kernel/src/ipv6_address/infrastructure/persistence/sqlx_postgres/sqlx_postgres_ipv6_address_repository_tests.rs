use kernel::ipv6_address::domain::errors::ipv6_address_repository_error::Ipv6AddressRepositoryError;
use kernel::ipv6_address::domain::repositories::ipv6_address_repository::Ipv6AddressRepository;
use kernel::ipv6_address::infrastructure::persistence::sqlx_postgres::sqlx_postgres_ipv6_address_repository::SqlxPostgresIpv6AddressRepository;
use sqlx::PgPool;

use crate::src::ipv6_address::domain::entities::mothers::ipv6_address_mother::Ipv6AddressMother;
use crate::src::ipv6_address::domain::value_objects::mothers::ipv6_address_id_mother::Ipv6AddressIdMother;
use crate::src::ipv6_address::infrastructure::persistence::ipv6_address_repository_contract as contract;

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_finds_a_saved_ipv6_address(pool: PgPool) {
    contract::it_finds_a_saved_ipv6_address(&SqlxPostgresIpv6AddressRepository::new(pool)).await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_rejects_registering_the_same_address_twice(pool: PgPool) {
    contract::it_rejects_registering_the_same_address_twice(
        &SqlxPostgresIpv6AddressRepository::new(pool),
    )
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_returns_none_for_an_unknown_id(pool: PgPool) {
    contract::it_returns_none_for_an_unknown_id(&SqlxPostgresIpv6AddressRepository::new(pool))
        .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_deletes_a_saved_ipv6_address(pool: PgPool) {
    contract::it_deletes_a_saved_ipv6_address(&SqlxPostgresIpv6AddressRepository::new(pool)).await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_returns_not_found_when_deleting_an_unknown_id(pool: PgPool) {
    contract::it_returns_not_found_when_deleting_an_unknown_id(
        &SqlxPostgresIpv6AddressRepository::new(pool),
    )
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_accepts_a_deleted_address_again(pool: PgPool) {
    contract::it_accepts_a_deleted_address_again(&SqlxPostgresIpv6AddressRepository::new(pool))
        .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_reports_a_row_whose_id_does_not_match_its_value(pool: PgPool) {
    let tampered = Ipv6AddressIdMother::create("2001:db8:85a3::8a2e:370:7334");
    sqlx::query(
        "INSERT INTO kernel.ipv6_address (id, value, created_at) VALUES ($1, 'fe80::1', now())",
    )
    .bind(*tampered.value())
    .execute(&pool)
    .await
    .unwrap();

    let result = SqlxPostgresIpv6AddressRepository::new(pool)
        .find_by_id(&tampered)
        .await;

    assert!(matches!(
        result,
        Err(Ipv6AddressRepositoryError::Unexpected(_))
    ));
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_stores_the_address_as_a_host_inet(pool: PgPool) {
    let repository = SqlxPostgresIpv6AddressRepository::new(pool.clone());
    repository
        .save(&Ipv6AddressMother::create("FE80:0:0:0:0:0:0:1/128"))
        .await
        .unwrap();

    let stored: String = sqlx::query_scalar("SELECT value::text FROM kernel.ipv6_address")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(stored, "fe80::1/128");
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_refuses_a_value_that_is_not_an_ipv6_host_in_the_table(pool: PgPool) {
    for value in ["2001:db8::/32", "192.0.2.1"] {
        let result = sqlx::query(
            "INSERT INTO kernel.ipv6_address (id, value, created_at) VALUES ($1, $2::inet, now())",
        )
        .bind(*Ipv6AddressIdMother::create("fe80::1").value())
        .bind(value)
        .execute(&pool)
        .await;

        let error = result.expect_err("the table must refuse a value that is not an IPv6 host");
        assert_eq!(
            error.as_database_error().and_then(|e| e.constraint()),
            Some("ipv6_address_value_is_host"),
            "{value}"
        );
    }
}
