//! The command bus decorator that commits state and events together.

use std::any::Any;
use std::sync::Arc;

use async_trait::async_trait;
use shared_cqrs::command::domain::command::Command;
use shared_cqrs::command::domain::command_bus::CommandBus;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_domain_events::domain::event_bus::EventBus;
use sqlx::PgPool;
use tracing::warn;

use crate::infrastructure::postgres::pg_event_store::PgEventStore;
use crate::infrastructure::postgres::pg_unit_of_work::PgUnitOfWork;

/// A [`CommandBus`] that runs every dispatch inside a [`PgUnitOfWork`].
///
/// After the inner bus returns, it rolls back if the dispatch failed, a
/// statement failed or no event was published, and returns the handler's
/// response unchanged. Otherwise it appends the published events to the
/// event log, commits, hands the events to the in-process `after_commit`
/// bus (best effort) and returns the response.
///
/// "Commit only when events were published" relies on the Fennec rule that
/// every successful write publishes an event and a failed one publishes
/// none: business errors travel inside the response envelope, which this
/// decorator cannot inspect.
///
/// If appending the events or committing fails, the handler's response,
/// which already claims success, is discarded and the dispatch fails with
/// [`CommandBusError::HandlerError`]. This is the one deliberate exception
/// to "bus errors are reserved for wiring bugs".
pub struct TransactionalCommandBus {
    inner: Arc<dyn CommandBus>,
    pool: PgPool,
    after_commit: Arc<dyn EventBus>,
}

impl TransactionalCommandBus {
    pub fn new(inner: Arc<dyn CommandBus>, pool: PgPool, after_commit: Arc<dyn EventBus>) -> Self {
        Self {
            inner,
            pool,
            after_commit,
        }
    }
}

#[async_trait]
impl CommandBus for TransactionalCommandBus {
    async fn dispatch(
        &self,
        command: Box<dyn Command>,
    ) -> Result<Box<dyn Any + Send + Sync>, CommandBusError> {
        let unit_of_work = PgUnitOfWork::begin(&self.pool).await.map_err(|error| {
            CommandBusError::HandlerError(format!("could not begin a unit of work: {error}"))
        })?;

        let response = Arc::clone(&unit_of_work)
            .scope(self.inner.dispatch(command))
            .await;

        let events = unit_of_work.take_pending_events();
        let Some(mut transaction) = unit_of_work.take_transaction().await else {
            return Err(CommandBusError::HandlerError(
                "the unit of work was closed before the command finished".to_string(),
            ));
        };

        if response.is_err() || unit_of_work.has_failed() || events.is_empty() {
            if let Err(error) = transaction.rollback().await {
                warn!(error = %error, "Failed to roll back a unit of work");
            }
            return response;
        }

        if let Err(error) = PgEventStore::append(&mut transaction, &events).await {
            if let Err(rollback_error) = transaction.rollback().await {
                warn!(error = %rollback_error, "Failed to roll back a unit of work");
            }
            return Err(CommandBusError::HandlerError(format!(
                "could not append the command's events: {error}"
            )));
        }

        transaction.commit().await.map_err(|error| {
            CommandBusError::HandlerError(format!("could not commit the unit of work: {error}"))
        })?;

        if let Err(error) = self.after_commit.publish(events) {
            warn!(error = %error, "An in-process subscriber failed after commit");
        }

        response
    }
}
