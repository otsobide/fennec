use std::sync::Arc;

use async_trait::async_trait;
use shared_cqrs::command::domain::command::Command;
use shared_cqrs::command::domain::command_bus::CommandBus;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;
use shared_cqrs::command::infrastructure::in_memory::in_memory_command_bus::InMemoryCommandBus;
use shared_domain_events::domain::event_bus::EventBus;
use shared_persistence_postgres::infrastructure::postgres::aggregate_lock::AggregateLock;
use shared_persistence_postgres::infrastructure::postgres::outbox_event_bus::OutboxEventBus;
use shared_persistence_postgres::infrastructure::postgres::pg_session::PgSession;
use shared_persistence_postgres::infrastructure::postgres::transactional_command_bus::TransactionalCommandBus;
use sqlx::PgPool;
use uuid::Uuid;

use crate::src::support::recording_event_bus::RecordingEventBus;
use crate::src::support::test_event::{TestEvent, TEST_EVENT_NAME};

/// What the test handler does after writing its row.
#[derive(Clone, Copy)]
enum Outcome {
    PublishesAnEvent,
    PublishesNothing,
    FailsAStatement,
    PublishesAnUnstorableEvent,
}

struct CreateWidgetCommand {
    id: Uuid,
    outcome: Outcome,
}

impl Command for CreateWidgetCommand {}

struct CreateWidgetHandler {
    session: PgSession,
    event_bus: Arc<dyn EventBus>,
}

#[async_trait]
impl CommandHandler<CreateWidgetCommand> for CreateWidgetHandler {
    type Response = &'static str;

    async fn handle(
        &self,
        command: CreateWidgetCommand,
    ) -> Result<Self::Response, CommandBusError> {
        let id = command.id;
        let written = self
            .session
            .run(AggregateLock::new("widget", &id), move |connection| {
                Box::pin(async move {
                    sqlx::query("INSERT INTO widget (id) VALUES ($1)")
                        .bind(id)
                        .execute(connection)
                        .await
                })
            })
            .await;
        if written.is_err() {
            return Ok("write failed");
        }

        match command.outcome {
            Outcome::PublishesAnEvent => {
                self.event_bus
                    .publish(vec![Box::new(TestEvent::new(id.to_string(), "widget"))])
                    .unwrap();
            }
            Outcome::PublishesNothing => {}
            Outcome::FailsAStatement => {
                let _ = self
                    .session
                    .run(AggregateLock::new("widget", &id), |connection| {
                        Box::pin(async move {
                            sqlx::query("SELECT * FROM no_such_table")
                                .execute(connection)
                                .await
                        })
                    })
                    .await;
                self.event_bus
                    .publish(vec![Box::new(TestEvent::new(id.to_string(), "widget"))])
                    .unwrap();
            }
            Outcome::PublishesAnUnstorableEvent => {
                self.event_bus
                    .publish(vec![Box::new(TestEvent::new("not-a-uuid", "widget"))])
                    .unwrap();
            }
        }

        Ok("handled")
    }
}

struct Harness {
    bus: TransactionalCommandBus,
    after_commit: Arc<RecordingEventBus>,
    pool: PgPool,
}

async fn harness(pool: PgPool) -> Harness {
    sqlx::query("CREATE TABLE widget (id uuid PRIMARY KEY)")
        .execute(&pool)
        .await
        .unwrap();

    let mut inner = InMemoryCommandBus::new();
    inner
        .register(CreateWidgetHandler {
            session: PgSession::new(pool.clone()),
            event_bus: Arc::new(OutboxEventBus::new()),
        })
        .unwrap();
    let after_commit = Arc::new(RecordingEventBus::default());
    let after_commit_bus: Arc<dyn EventBus> = after_commit.clone();

    Harness {
        bus: TransactionalCommandBus::new(Arc::new(inner), pool.clone(), after_commit_bus),
        after_commit,
        pool,
    }
}

impl Harness {
    async fn dispatch(&self, outcome: Outcome) -> Result<&'static str, CommandBusError> {
        let command = CreateWidgetCommand {
            id: Uuid::new_v4(),
            outcome,
        };
        self.bus
            .dispatch(Box::new(command))
            .await
            .map(|response| *response.downcast::<&'static str>().unwrap())
    }

    async fn widgets(&self) -> i64 {
        sqlx::query_scalar("SELECT count(*) FROM widget")
            .fetch_one(&self.pool)
            .await
            .unwrap()
    }

    async fn events(&self) -> i64 {
        sqlx::query_scalar("SELECT count(*) FROM event_store.domain_event")
            .fetch_one(&self.pool)
            .await
            .unwrap()
    }
}

#[sqlx::test(
    migrator = "shared_persistence_postgres::infrastructure::postgres::event_store_migrator::EVENT_STORE_MIGRATOR"
)]
#[ignore = "requires DATABASE_URL"]
async fn it_commits_the_state_and_the_events_together(pool: PgPool) {
    let harness = harness(pool).await;

    let response = harness.dispatch(Outcome::PublishesAnEvent).await;

    assert_eq!(response.unwrap(), "handled");
    assert_eq!(harness.widgets().await, 1);
    assert_eq!(harness.events().await, 1);
}

#[sqlx::test(
    migrator = "shared_persistence_postgres::infrastructure::postgres::event_store_migrator::EVENT_STORE_MIGRATOR"
)]
#[ignore = "requires DATABASE_URL"]
async fn it_delivers_the_events_in_process_after_the_commit(pool: PgPool) {
    let harness = harness(pool).await;

    harness.dispatch(Outcome::PublishesAnEvent).await.unwrap();

    assert_eq!(
        harness.after_commit.published_event_names(),
        vec![TEST_EVENT_NAME]
    );
}

#[sqlx::test(
    migrator = "shared_persistence_postgres::infrastructure::postgres::event_store_migrator::EVENT_STORE_MIGRATOR"
)]
#[ignore = "requires DATABASE_URL"]
async fn it_rolls_back_a_command_that_publishes_no_event(pool: PgPool) {
    let harness = harness(pool).await;

    let response = harness.dispatch(Outcome::PublishesNothing).await;

    assert_eq!(response.unwrap(), "handled");
    assert_eq!(harness.widgets().await, 0);
    assert_eq!(harness.events().await, 0);
    assert!(harness.after_commit.published_event_names().is_empty());
}

#[sqlx::test(
    migrator = "shared_persistence_postgres::infrastructure::postgres::event_store_migrator::EVENT_STORE_MIGRATOR"
)]
#[ignore = "requires DATABASE_URL"]
async fn it_rolls_back_a_command_whose_statement_failed(pool: PgPool) {
    let harness = harness(pool).await;

    harness.dispatch(Outcome::FailsAStatement).await.unwrap();

    assert_eq!(harness.widgets().await, 0);
    assert_eq!(harness.events().await, 0);
}

#[sqlx::test(
    migrator = "shared_persistence_postgres::infrastructure::postgres::event_store_migrator::EVENT_STORE_MIGRATOR"
)]
#[ignore = "requires DATABASE_URL"]
async fn it_fails_the_dispatch_when_the_events_cannot_be_stored(pool: PgPool) {
    let harness = harness(pool).await;

    let response = harness.dispatch(Outcome::PublishesAnUnstorableEvent).await;

    assert!(matches!(response, Err(CommandBusError::HandlerError(_))));
    assert_eq!(harness.widgets().await, 0);
    assert_eq!(harness.events().await, 0);
}
