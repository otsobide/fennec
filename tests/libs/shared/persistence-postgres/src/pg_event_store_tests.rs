use shared_domain_events::domain::domain_event::DomainEvent;
use shared_persistence_postgres::infrastructure::postgres::pg_event_store::PgEventStore;
use sqlx::{PgPool, Row};

use crate::src::support::test_event::{TestEvent, TEST_EVENT_NAME};

const AGGREGATE_ID: &str = "8b3d1c58-5f0b-4a43-9d4e-0f4b8d0c9a11";

#[sqlx::test(
    migrator = "shared_persistence_postgres::infrastructure::postgres::event_store_migrator::EVENT_STORE_MIGRATOR"
)]
#[ignore = "requires DATABASE_URL"]
async fn it_appends_an_event_with_its_payload(pool: PgPool) {
    let event = TestEvent::new(AGGREGATE_ID, "widget");
    let event_id = event.event_id().to_string();
    let mut connection = pool.acquire().await.unwrap();

    PgEventStore::append(&mut connection, &[Box::new(event)])
        .await
        .unwrap();

    let row = sqlx::query(
        "SELECT event_id::text, event_name, aggregate_type, aggregate_id::text, payload::text
         FROM event_store.domain_event",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(row.get::<String, _>(0), event_id);
    assert_eq!(row.get::<String, _>(1), TEST_EVENT_NAME);
    assert_eq!(row.get::<String, _>(2), "widget");
    assert_eq!(row.get::<String, _>(3), AGGREGATE_ID);
    assert_eq!(
        row.get::<String, _>(4),
        r#"{"id": "8b3d1c58-5f0b-4a43-9d4e-0f4b8d0c9a11", "name": "widget", "count": 3, "created_at": "2025-06-15T15:06:40.123456Z"}"#
    );
}

#[sqlx::test(
    migrator = "shared_persistence_postgres::infrastructure::postgres::event_store_migrator::EVENT_STORE_MIGRATOR"
)]
#[ignore = "requires DATABASE_URL"]
async fn it_keeps_the_order_of_a_batch(pool: PgPool) {
    let events: Vec<Box<dyn DomainEvent>> = ["first", "second", "third"]
        .into_iter()
        .map(|name| Box::new(TestEvent::new(AGGREGATE_ID, name)) as Box<dyn DomainEvent>)
        .collect();
    let mut connection = pool.acquire().await.unwrap();

    PgEventStore::append(&mut connection, &events)
        .await
        .unwrap();

    let names: Vec<String> = sqlx::query_scalar(
        "SELECT payload->>'name' FROM event_store.domain_event ORDER BY position",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(names, vec!["first", "second", "third"]);
}

#[sqlx::test(
    migrator = "shared_persistence_postgres::infrastructure::postgres::event_store_migrator::EVENT_STORE_MIGRATOR"
)]
#[ignore = "requires DATABASE_URL"]
async fn it_rejects_an_event_whose_aggregate_id_is_not_a_uuid(pool: PgPool) {
    let mut connection = pool.acquire().await.unwrap();

    let result = PgEventStore::append(
        &mut connection,
        &[Box::new(TestEvent::new("not-a-uuid", "widget"))],
    )
    .await;

    assert!(matches!(result, Err(sqlx::Error::Encode(_))));
}
