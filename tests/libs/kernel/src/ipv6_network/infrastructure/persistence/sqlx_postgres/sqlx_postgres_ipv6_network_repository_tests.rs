use kernel::ipv6_network::domain::errors::ipv6_network_repository_error::Ipv6NetworkRepositoryError;
use kernel::ipv6_network::domain::repositories::ipv6_network_repository::Ipv6NetworkRepository;
use kernel::ipv6_network::infrastructure::persistence::sqlx_postgres::sqlx_postgres_ipv6_network_repository::SqlxPostgresIpv6NetworkRepository;
use sqlx::PgPool;

use crate::src::ipv6_network::domain::entities::mothers::ipv6_network_mother::Ipv6NetworkMother;
use crate::src::ipv6_network::domain::value_objects::mothers::ipv6_network_id_mother::Ipv6NetworkIdMother;
use crate::src::ipv6_network::infrastructure::persistence::ipv6_network_repository_contract as contract;

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_finds_a_saved_ipv6_network(pool: PgPool) {
    contract::it_finds_a_saved_ipv6_network(&SqlxPostgresIpv6NetworkRepository::new(pool)).await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_rejects_registering_the_same_network_twice(pool: PgPool) {
    contract::it_rejects_registering_the_same_network_twice(
        &SqlxPostgresIpv6NetworkRepository::new(pool),
    )
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_returns_none_for_an_unknown_id(pool: PgPool) {
    contract::it_returns_none_for_an_unknown_id(&SqlxPostgresIpv6NetworkRepository::new(pool))
        .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_deletes_a_saved_ipv6_network(pool: PgPool) {
    contract::it_deletes_a_saved_ipv6_network(&SqlxPostgresIpv6NetworkRepository::new(pool)).await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_returns_not_found_when_deleting_an_unknown_id(pool: PgPool) {
    contract::it_returns_not_found_when_deleting_an_unknown_id(
        &SqlxPostgresIpv6NetworkRepository::new(pool),
    )
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_accepts_a_deleted_network_again(pool: PgPool) {
    contract::it_accepts_a_deleted_network_again(&SqlxPostgresIpv6NetworkRepository::new(pool))
        .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_reports_a_row_whose_id_does_not_match_its_value(pool: PgPool) {
    let tampered = Ipv6NetworkIdMother::create("2001:db8::/32");
    sqlx::query(
        "INSERT INTO kernel.ipv6_network (id, value, created_at) VALUES ($1, $2::cidr, now())",
    )
    .bind(*tampered.value())
    .bind("fd12:3456::/32")
    .execute(&pool)
    .await
    .unwrap();

    let result = SqlxPostgresIpv6NetworkRepository::new(pool)
        .find_by_id(&tampered)
        .await;

    assert!(matches!(
        result,
        Err(Ipv6NetworkRepositoryError::Unexpected(_))
    ));
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_stores_the_canonical_value(pool: PgPool) {
    let repository = SqlxPostgresIpv6NetworkRepository::new(pool.clone());
    repository
        .save(&Ipv6NetworkMother::create("FD12:3456:0:0:0:0:0:0/32"))
        .await
        .unwrap();

    let stored: String = sqlx::query_scalar("SELECT value::text FROM kernel.ipv6_network")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(stored, "fd12:3456::/32");
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_lets_the_column_type_canonicalise_a_raw_value(pool: PgPool) {
    sqlx::query(
        "INSERT INTO kernel.ipv6_network (id, value, created_at) VALUES ($1, $2::cidr, now())",
    )
    .bind(*Ipv6NetworkIdMother::create("2001:db8::/32").value())
    .bind("2001:0DB8:0000::/32")
    .execute(&pool)
    .await
    .unwrap();

    let stored: String = sqlx::query_scalar("SELECT value::text FROM kernel.ipv6_network")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(stored, "2001:db8::/32");
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_refuses_a_block_of_the_other_family_or_a_host_in_the_table(pool: PgPool) {
    for value in ["198.51.100.0/24", "2001:db8::7/128"] {
        let result = sqlx::query(
            "INSERT INTO kernel.ipv6_network (id, value, created_at) VALUES ($1, $2::cidr, now())",
        )
        .bind(*Ipv6NetworkIdMother::create("2001:db8::/32").value())
        .bind(value)
        .execute(&pool)
        .await;

        let error = result.expect_err("the table must refuse the block");
        assert_eq!(
            error.as_database_error().and_then(|e| e.constraint()),
            Some("ipv6_network_value_is_network"),
            "{value}"
        );
    }
}
