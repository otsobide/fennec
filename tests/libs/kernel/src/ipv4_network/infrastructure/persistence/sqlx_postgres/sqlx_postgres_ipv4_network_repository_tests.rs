use kernel::ipv4_network::domain::errors::ipv4_network_repository_error::Ipv4NetworkRepositoryError;
use kernel::ipv4_network::domain::repositories::ipv4_network_repository::Ipv4NetworkRepository;
use kernel::ipv4_network::infrastructure::persistence::sqlx_postgres::sqlx_postgres_ipv4_network_repository::SqlxPostgresIpv4NetworkRepository;
use sqlx::PgPool;

use crate::src::ipv4_network::domain::entities::mothers::ipv4_network_mother::Ipv4NetworkMother;
use crate::src::ipv4_network::domain::value_objects::mothers::ipv4_network_id_mother::Ipv4NetworkIdMother;
use crate::src::ipv4_network::infrastructure::persistence::ipv4_network_repository_contract as contract;

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_finds_a_saved_ipv4_network(pool: PgPool) {
    contract::it_finds_a_saved_ipv4_network(&SqlxPostgresIpv4NetworkRepository::new(pool)).await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_rejects_registering_the_same_network_twice(pool: PgPool) {
    contract::it_rejects_registering_the_same_network_twice(
        &SqlxPostgresIpv4NetworkRepository::new(pool),
    )
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_returns_none_for_an_unknown_id(pool: PgPool) {
    contract::it_returns_none_for_an_unknown_id(&SqlxPostgresIpv4NetworkRepository::new(pool))
        .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_deletes_a_saved_ipv4_network(pool: PgPool) {
    contract::it_deletes_a_saved_ipv4_network(&SqlxPostgresIpv4NetworkRepository::new(pool)).await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_returns_not_found_when_deleting_an_unknown_id(pool: PgPool) {
    contract::it_returns_not_found_when_deleting_an_unknown_id(
        &SqlxPostgresIpv4NetworkRepository::new(pool),
    )
    .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_accepts_a_deleted_network_again(pool: PgPool) {
    contract::it_accepts_a_deleted_network_again(&SqlxPostgresIpv4NetworkRepository::new(pool))
        .await;
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_reports_a_row_whose_id_does_not_match_its_value(pool: PgPool) {
    let tampered = Ipv4NetworkIdMother::create("198.51.100.0/24");
    sqlx::query(
        "INSERT INTO kernel.ipv4_network (id, value, created_at) VALUES ($1, $2::cidr, now())",
    )
    .bind(*tampered.value())
    .bind("10.0.0.0/8")
    .execute(&pool)
    .await
    .unwrap();

    let result = SqlxPostgresIpv4NetworkRepository::new(pool)
        .find_by_id(&tampered)
        .await;

    assert!(matches!(
        result,
        Err(Ipv4NetworkRepositoryError::Unexpected(_))
    ));
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_stores_the_canonical_value(pool: PgPool) {
    let repository = SqlxPostgresIpv4NetworkRepository::new(pool.clone());
    repository
        .save(&Ipv4NetworkMother::create("  10.0.0.0/8"))
        .await
        .unwrap();

    let stored: String = sqlx::query_scalar("SELECT value::text FROM kernel.ipv4_network")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(stored, "10.0.0.0/8");
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_lets_the_column_type_canonicalise_a_raw_value(pool: PgPool) {
    sqlx::query(
        "INSERT INTO kernel.ipv4_network (id, value, created_at) VALUES ($1, $2::cidr, now())",
    )
    .bind(*Ipv4NetworkIdMother::create("198.51.100.0/24").value())
    .bind("198.51.100/24")
    .execute(&pool)
    .await
    .unwrap();

    let stored: String = sqlx::query_scalar("SELECT value::text FROM kernel.ipv4_network")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(stored, "198.51.100.0/24");
}

#[sqlx::test(migrator = "kernel::kernel_migrator::KERNEL_MIGRATOR")]
#[ignore = "requires DATABASE_URL"]
async fn it_refuses_a_block_of_the_other_family_or_a_host_in_the_table(pool: PgPool) {
    for value in ["2001:db8::/32", "198.51.100.7/32"] {
        let result = sqlx::query(
            "INSERT INTO kernel.ipv4_network (id, value, created_at) VALUES ($1, $2::cidr, now())",
        )
        .bind(*Ipv4NetworkIdMother::create("198.51.100.0/24").value())
        .bind(value)
        .execute(&pool)
        .await;

        let error = result.expect_err("the table must refuse the block");
        assert_eq!(
            error.as_database_error().and_then(|e| e.constraint()),
            Some("ipv4_network_value_is_network"),
            "{value}"
        );
    }
}
