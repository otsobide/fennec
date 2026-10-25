use shared_domain_events::domain::event_bus::EventBus;
use shared_persistence_postgres::infrastructure::postgres::outbox_event_bus::OutboxEventBus;
use shared_persistence_postgres::infrastructure::postgres::pg_unit_of_work::PgUnitOfWork;
use sqlx::PgPool;

use crate::src::support::test_event::{TestEvent, TEST_EVENT_NAME};

#[test]
fn it_rejects_publishing_outside_a_unit_of_work() {
    let result = OutboxEventBus::new().publish(vec![Box::new(TestEvent::new(
        "8b3d1c58-5f0b-4a43-9d4e-0f4b8d0c9a11",
        "widget",
    ))]);

    assert!(result.is_err());
}

#[sqlx::test]
#[ignore = "requires DATABASE_URL"]
async fn it_buffers_events_in_the_current_unit_of_work(pool: PgPool) {
    let unit_of_work = PgUnitOfWork::begin(&pool).await.unwrap();

    let result = std::sync::Arc::clone(&unit_of_work)
        .scope(async {
            OutboxEventBus::new().publish(vec![Box::new(TestEvent::new(
                "8b3d1c58-5f0b-4a43-9d4e-0f4b8d0c9a11",
                "widget",
            ))])
        })
        .await;

    assert!(result.is_ok());
    let pending = unit_of_work.take_pending_events();
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].event_name(), TEST_EVENT_NAME);
}
