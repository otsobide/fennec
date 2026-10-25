use std::time::Duration;

use shared_persistence_postgres::infrastructure::postgres::aggregate_lock::AggregateLock;
use sqlx::PgPool;
use uuid::Uuid;

fn ipv4_id() -> Uuid {
    Uuid::parse_str("abfb438a-8baf-5f54-b19e-b85ee74b3ed7").unwrap()
}

#[test]
fn it_derives_the_key_from_the_aggregate_type_and_id() {
    assert_eq!(
        AggregateLock::new("ipv4_address", &ipv4_id()).key(),
        -7_439_548_489_386_278_439
    );
}

#[test]
fn it_gives_aggregates_of_different_kinds_different_keys() {
    assert_ne!(
        AggregateLock::new("ipv4_address", &ipv4_id()),
        AggregateLock::new("hostname", &ipv4_id())
    );
}

#[sqlx::test]
#[ignore = "requires DATABASE_URL"]
async fn it_assigns_no_transaction_id(pool: PgPool) {
    let mut transaction = pool.begin().await.unwrap();

    AggregateLock::new("ipv4_address", &ipv4_id())
        .acquire(&mut transaction)
        .await
        .unwrap();
    let assigned: Option<String> =
        sqlx::query_scalar("SELECT pg_current_xact_id_if_assigned()::text")
            .fetch_one(&mut *transaction)
            .await
            .unwrap();

    assert_eq!(assigned, None);
}

#[sqlx::test]
#[ignore = "requires DATABASE_URL"]
async fn it_makes_a_second_transaction_wait_for_the_first(pool: PgPool) {
    let lock = AggregateLock::new("ipv4_address", &ipv4_id());
    let mut first = pool.begin().await.unwrap();
    lock.acquire(&mut first).await.unwrap();

    let mut second = pool.begin().await.unwrap();
    sqlx::query("SET LOCAL lock_timeout = '200ms'")
        .execute(&mut *second)
        .await
        .unwrap();
    let blocked = lock.acquire(&mut second).await;
    assert!(blocked.is_err(), "the second transaction must wait");
    second.rollback().await.unwrap();

    first.commit().await.unwrap();
    let mut third = pool.begin().await.unwrap();
    tokio::time::timeout(Duration::from_secs(5), lock.acquire(&mut third))
        .await
        .expect("the lock is free after the first commit")
        .unwrap();
}

#[sqlx::test]
#[ignore = "requires DATABASE_URL"]
async fn it_can_be_taken_again_in_the_same_transaction(pool: PgPool) {
    let lock = AggregateLock::new("ipv4_address", &ipv4_id());
    let mut transaction = pool.begin().await.unwrap();

    lock.acquire(&mut transaction).await.unwrap();
    lock.acquire(&mut transaction).await.unwrap();
}
