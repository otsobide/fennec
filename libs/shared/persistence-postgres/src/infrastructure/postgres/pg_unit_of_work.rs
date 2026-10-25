//! The transaction and the pending events of one command.

use std::future::Future;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex as StdMutex};

use shared_domain_events::domain::domain_event::DomainEvent;
use sqlx::{PgConnection, PgPool, Postgres, Transaction};
use tokio::sync::Mutex;

use crate::infrastructure::postgres::pg_operation::PgOperation;

tokio::task_local! {
    static CURRENT_UNIT_OF_WORK: Arc<PgUnitOfWork>;
}

/// The unit of work of one command: a PostgreSQL transaction, the domain
/// events published while it is open, and whether any statement failed.
///
/// [`TransactionalCommandBus`] opens one per dispatch and makes it current
/// for the task that runs the handler, so repositories and the outbox event
/// bus find it without any change to services or ports. Everything a
/// command writes, and the events it publishes, commit together.
///
/// [`TransactionalCommandBus`]: crate::infrastructure::postgres::transactional_command_bus::TransactionalCommandBus
pub struct PgUnitOfWork {
    transaction: Mutex<Option<Transaction<'static, Postgres>>>,
    pending_events: StdMutex<Vec<Box<dyn DomainEvent>>>,
    failed: AtomicBool,
}

impl PgUnitOfWork {
    /// Begins a transaction on the pool.
    ///
    /// # Errors
    ///
    /// Returns the driver error if no connection or transaction can be
    /// obtained.
    pub async fn begin(pool: &PgPool) -> Result<Arc<Self>, sqlx::Error> {
        let transaction = pool.begin().await?;

        Ok(Arc::new(Self {
            transaction: Mutex::new(Some(transaction)),
            pending_events: StdMutex::new(Vec::new()),
            failed: AtomicBool::new(false),
        }))
    }

    /// Returns the unit of work of the current task, if one is in scope.
    pub fn current() -> Option<Arc<Self>> {
        CURRENT_UNIT_OF_WORK.try_with(Arc::clone).ok()
    }

    /// Runs `future` with this unit of work as the current one.
    pub async fn scope<F: Future>(self: Arc<Self>, future: F) -> F::Output {
        CURRENT_UNIT_OF_WORK.scope(self, future).await
    }

    /// Runs `operation` on the transaction, marking the unit of work as
    /// failed if the operation fails, so it is rolled back.
    ///
    /// # Errors
    ///
    /// Returns the operation's error, or [`sqlx::Error::Protocol`] if the
    /// transaction has already been taken.
    pub async fn execute<T, F>(&self, operation: F) -> Result<T, sqlx::Error>
    where
        F: for<'c> FnOnce(&'c mut PgConnection) -> PgOperation<'c, T>,
    {
        let mut guard = self.transaction.lock().await;
        let Some(transaction) = guard.as_mut() else {
            self.mark_failed();
            return Err(sqlx::Error::Protocol(
                "the unit of work is already closed".to_string(),
            ));
        };

        let result = operation(&mut **transaction).await;
        if result.is_err() {
            self.mark_failed();
        }
        result
    }

    /// Records that a statement failed, so the unit of work must roll back.
    pub fn mark_failed(&self) {
        self.failed.store(true, Ordering::SeqCst);
    }

    /// Returns `true` if any statement of the unit of work failed.
    pub fn has_failed(&self) -> bool {
        self.failed.load(Ordering::SeqCst)
    }

    /// Buffers events to be appended to the event log at commit.
    pub fn buffer_events(&self, events: Vec<Box<dyn DomainEvent>>) {
        self.pending_events
            .lock()
            .expect("the pending events lock is never poisoned")
            .extend(events);
    }

    /// Takes the buffered events, leaving none.
    pub fn take_pending_events(&self) -> Vec<Box<dyn DomainEvent>> {
        std::mem::take(
            &mut *self
                .pending_events
                .lock()
                .expect("the pending events lock is never poisoned"),
        )
    }

    /// Takes the transaction to commit or roll it back. Later operations fail.
    pub async fn take_transaction(&self) -> Option<Transaction<'static, Postgres>> {
        self.transaction.lock().await.take()
    }
}
