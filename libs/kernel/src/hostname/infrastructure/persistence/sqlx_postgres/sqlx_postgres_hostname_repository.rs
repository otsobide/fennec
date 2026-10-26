//! PostgreSQL implementation of [`HostnameRepository`].

use async_trait::async_trait;
use shared_persistence_postgres::infrastructure::postgres::aggregate_lock::AggregateLock;
use shared_persistence_postgres::infrastructure::postgres::pg_session::PgSession;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::hostname::domain::entities::hostname::Hostname;
use crate::hostname::domain::errors::hostname_repository_error::HostnameRepositoryError;
use crate::hostname::domain::repositories::hostname_repository::HostnameRepository;
use crate::hostname::domain::value_objects::hostname_created_at::HostnameCreatedAt;
use crate::hostname::domain::value_objects::hostname_id::HostnameId;
use crate::hostname::domain::value_objects::hostname_value::HostnameValue;

/// The aggregate type of the advisory lock and of the event log rows.
const AGGREGATE_TYPE: &str = "hostname";

const INSERT: &str = "
    INSERT INTO kernel.hostname (id, value, created_at)
    VALUES ($1, $2, $3)
    ON CONFLICT DO NOTHING";

const SELECT_BY_ID: &str = "SELECT id, value, created_at FROM kernel.hostname WHERE id = $1";

const DELETE: &str = "DELETE FROM kernel.hostname WHERE id = $1";

/// [`HostnameRepository`] backed by the `kernel.hostname` table.
///
/// Writes run on the current unit of work after taking the hostname's
/// advisory lock, so they commit with the command's events. A duplicate is
/// detected with `ON CONFLICT DO NOTHING`, which never aborts the unit of
/// work. Rows are rebuilt through the value objects, and a row whose id is
/// not the one derived from its value is reported as `Unexpected`.
pub struct SqlxPostgresHostnameRepository {
    session: PgSession,
}

impl SqlxPostgresHostnameRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            session: PgSession::new(pool),
        }
    }
}

#[async_trait]
impl HostnameRepository for SqlxPostgresHostnameRepository {
    async fn save(&self, hostname: &Hostname) -> Result<(), HostnameRepositoryError> {
        let id = *hostname.id().value();
        let value = hostname.value().to_string();
        let created_at = to_offset_date_time(hostname.created_at())?;

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
            return Err(HostnameRepositoryError::AlreadyExists);
        }
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &HostnameId,
    ) -> Result<Option<Hostname>, HostnameRepositoryError> {
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

    async fn delete(&self, id: &HostnameId) -> Result<(), HostnameRepositoryError> {
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
            return Err(HostnameRepositoryError::NotFound);
        }
        Ok(())
    }
}

fn lock(id: &Uuid) -> AggregateLock {
    AggregateLock::new(AGGREGATE_TYPE, id)
}

fn to_offset_date_time(
    created_at: &HostnameCreatedAt,
) -> Result<OffsetDateTime, HostnameRepositoryError> {
    OffsetDateTime::from_unix_timestamp_nanos(i128::from(created_at.unix_micros()) * 1_000)
        .map_err(|error| HostnameRepositoryError::Unexpected(error.to_string()))
}

fn to_aggregate(
    id: Uuid,
    value: String,
    created_at: OffsetDateTime,
) -> Result<Hostname, HostnameRepositoryError> {
    let stored = HostnameValue::new(&value).map_err(|error| {
        HostnameRepositoryError::Unexpected(format!(
            "kernel.hostname row {id} holds an invalid value: {error}"
        ))
    })?;
    let micros = u64::try_from(created_at.unix_timestamp_nanos() / 1_000).map_err(|_| {
        HostnameRepositoryError::Unexpected(format!(
            "kernel.hostname row {id} was created before the Unix epoch"
        ))
    })?;
    let created_at = HostnameCreatedAt::from_unix_micros(micros)
        .map_err(|error| HostnameRepositoryError::Unexpected(error.to_string()))?;

    let hostname = Hostname::new(stored, created_at);
    if *hostname.id().value() != id || hostname.value().to_string() != value {
        return Err(HostnameRepositoryError::Unexpected(format!(
            "kernel.hostname row {id} is not stored in canonical form under the id its value derives"
        )));
    }
    Ok(hostname)
}

fn unexpected(error: sqlx::Error) -> HostnameRepositoryError {
    HostnameRepositoryError::Unexpected(error.to_string())
}
