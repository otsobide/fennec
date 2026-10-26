//! PostgreSQL implementation of [`EmailAddressRepository`].

use async_trait::async_trait;
use shared_persistence_postgres::infrastructure::postgres::aggregate_lock::AggregateLock;
use shared_persistence_postgres::infrastructure::postgres::pg_session::PgSession;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::email_address::domain::entities::email_address::EmailAddress;
use crate::email_address::domain::errors::email_address_repository_error::EmailAddressRepositoryError;
use crate::email_address::domain::repositories::email_address_repository::EmailAddressRepository;
use crate::email_address::domain::value_objects::email_address_created_at::EmailAddressCreatedAt;
use crate::email_address::domain::value_objects::email_address_id::EmailAddressId;
use crate::email_address::domain::value_objects::email_address_value::EmailAddressValue;

/// The aggregate type of the advisory lock and of the event log rows.
const AGGREGATE_TYPE: &str = "email_address";

const INSERT: &str = "
    INSERT INTO kernel.email_address (id, value, created_at)
    VALUES ($1, $2, $3)
    ON CONFLICT DO NOTHING";

const SELECT_BY_ID: &str = "SELECT id, value, created_at FROM kernel.email_address WHERE id = $1";

const DELETE: &str = "DELETE FROM kernel.email_address WHERE id = $1";

/// [`EmailAddressRepository`] backed by the `kernel.email_address` table.
///
/// Writes run on the current unit of work after taking the address's
/// advisory lock, so they commit with the command's events. A duplicate is
/// detected with `ON CONFLICT DO NOTHING`, which never aborts the unit of
/// work. Rows are rebuilt through the value objects, and a row whose id is
/// not the one derived from its value is reported as `Unexpected`.
pub struct SqlxPostgresEmailAddressRepository {
    session: PgSession,
}

impl SqlxPostgresEmailAddressRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            session: PgSession::new(pool),
        }
    }
}

#[async_trait]
impl EmailAddressRepository for SqlxPostgresEmailAddressRepository {
    async fn save(&self, email_address: &EmailAddress) -> Result<(), EmailAddressRepositoryError> {
        let id = *email_address.id().value();
        let value = email_address.value().to_string();
        let created_at = to_offset_date_time(email_address.created_at())?;

        let inserted = self
            .session
            .run(lock(&id), move |connection| {
                Box::pin(async move {
                    sqlx::query(INSERT)
                        .bind(id)
                        .bind(value)
                        .bind(created_at)
                        .execute(connection)
                        .await
                        .map(|result| result.rows_affected())
                })
            })
            .await
            .map_err(unexpected)?;

        if inserted == 0 {
            return Err(EmailAddressRepositoryError::AlreadyExists);
        }
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &EmailAddressId,
    ) -> Result<Option<EmailAddress>, EmailAddressRepositoryError> {
        let id = *id.value();

        let row = self
            .session
            .run(lock(&id), move |connection| {
                Box::pin(async move {
                    sqlx::query_as::<_, (Uuid, String, OffsetDateTime)>(SELECT_BY_ID)
                        .bind(id)
                        .fetch_optional(connection)
                        .await
                })
            })
            .await
            .map_err(unexpected)?;

        row.map(|(id, value, created_at)| to_aggregate(id, value, created_at))
            .transpose()
    }

    async fn delete(&self, id: &EmailAddressId) -> Result<(), EmailAddressRepositoryError> {
        let id = *id.value();

        let deleted = self
            .session
            .run(lock(&id), move |connection| {
                Box::pin(async move {
                    sqlx::query(DELETE)
                        .bind(id)
                        .execute(connection)
                        .await
                        .map(|result| result.rows_affected())
                })
            })
            .await
            .map_err(unexpected)?;

        if deleted == 0 {
            return Err(EmailAddressRepositoryError::NotFound);
        }
        Ok(())
    }
}

fn lock(id: &Uuid) -> AggregateLock {
    AggregateLock::new(AGGREGATE_TYPE, id)
}

fn to_offset_date_time(
    created_at: &EmailAddressCreatedAt,
) -> Result<OffsetDateTime, EmailAddressRepositoryError> {
    OffsetDateTime::from_unix_timestamp_nanos(i128::from(created_at.unix_micros()) * 1_000)
        .map_err(|error| EmailAddressRepositoryError::Unexpected(error.to_string()))
}

fn to_aggregate(
    id: Uuid,
    value: String,
    created_at: OffsetDateTime,
) -> Result<EmailAddress, EmailAddressRepositoryError> {
    let stored = EmailAddressValue::new(&value).map_err(|error| {
        EmailAddressRepositoryError::Unexpected(format!(
            "kernel.email_address row {id} holds an invalid value: {error}"
        ))
    })?;
    let micros = u64::try_from(created_at.unix_timestamp_nanos() / 1_000).map_err(|_| {
        EmailAddressRepositoryError::Unexpected(format!(
            "kernel.email_address row {id} was created before the Unix epoch"
        ))
    })?;
    let created_at = EmailAddressCreatedAt::from_unix_micros(micros)
        .map_err(|error| EmailAddressRepositoryError::Unexpected(error.to_string()))?;

    let email_address = EmailAddress::new(stored, created_at);
    if *email_address.id().value() != id || email_address.value().to_string() != value {
        return Err(EmailAddressRepositoryError::Unexpected(format!(
            "kernel.email_address row {id} is not stored in canonical form under the id its value derives"
        )));
    }
    Ok(email_address)
}

fn unexpected(error: sqlx::Error) -> EmailAddressRepositoryError {
    EmailAddressRepositoryError::Unexpected(error.to_string())
}
