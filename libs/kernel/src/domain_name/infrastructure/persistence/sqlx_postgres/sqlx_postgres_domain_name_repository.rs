//! PostgreSQL implementation of [`DomainNameRepository`].

use async_trait::async_trait;
use shared_persistence_postgres::infrastructure::postgres::aggregate_lock::AggregateLock;
use shared_persistence_postgres::infrastructure::postgres::pg_session::PgSession;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::domain_name::domain::entities::domain_name::DomainName;
use crate::domain_name::domain::errors::domain_name_repository_error::DomainNameRepositoryError;
use crate::domain_name::domain::repositories::domain_name_repository::DomainNameRepository;
use crate::domain_name::domain::value_objects::domain_name_created_at::DomainNameCreatedAt;
use crate::domain_name::domain::value_objects::domain_name_id::DomainNameId;
use crate::domain_name::domain::value_objects::domain_name_value::DomainNameValue;

/// The aggregate type of the advisory lock and of the event log rows.
const AGGREGATE_TYPE: &str = "domain_name";

const INSERT: &str = "
    INSERT INTO kernel.domain_name (id, value, created_at)
    VALUES ($1, $2, $3)
    ON CONFLICT DO NOTHING";

const SELECT_BY_ID: &str = "SELECT id, value, created_at FROM kernel.domain_name WHERE id = $1";

const DELETE: &str = "DELETE FROM kernel.domain_name WHERE id = $1";

/// [`DomainNameRepository`] backed by the `kernel.domain_name` table.
///
/// Writes run on the current unit of work after taking the name's
/// advisory lock, so they commit with the command's events. A duplicate is
/// detected with `ON CONFLICT DO NOTHING`, which never aborts the unit of
/// work. Rows are rebuilt through the value objects, and a row whose id is
/// not the one derived from its value is reported as `Unexpected`.
pub struct SqlxPostgresDomainNameRepository {
    session: PgSession,
}

impl SqlxPostgresDomainNameRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            session: PgSession::new(pool),
        }
    }
}

#[async_trait]
impl DomainNameRepository for SqlxPostgresDomainNameRepository {
    async fn save(&self, domain_name: &DomainName) -> Result<(), DomainNameRepositoryError> {
        let id = *domain_name.id().value();
        let value = domain_name.value().to_string();
        let created_at = to_offset_date_time(domain_name.created_at())?;

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
            return Err(DomainNameRepositoryError::AlreadyExists);
        }
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &DomainNameId,
    ) -> Result<Option<DomainName>, DomainNameRepositoryError> {
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

    async fn delete(&self, id: &DomainNameId) -> Result<(), DomainNameRepositoryError> {
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
            return Err(DomainNameRepositoryError::NotFound);
        }
        Ok(())
    }
}

fn lock(id: &Uuid) -> AggregateLock {
    AggregateLock::new(AGGREGATE_TYPE, id)
}

fn to_offset_date_time(
    created_at: &DomainNameCreatedAt,
) -> Result<OffsetDateTime, DomainNameRepositoryError> {
    OffsetDateTime::from_unix_timestamp_nanos(i128::from(created_at.unix_micros()) * 1_000)
        .map_err(|error| DomainNameRepositoryError::Unexpected(error.to_string()))
}

fn to_aggregate(
    id: Uuid,
    value: String,
    created_at: OffsetDateTime,
) -> Result<DomainName, DomainNameRepositoryError> {
    let stored = DomainNameValue::new(&value).map_err(|error| {
        DomainNameRepositoryError::Unexpected(format!(
            "kernel.domain_name row {id} holds an invalid value: {error}"
        ))
    })?;
    let micros = u64::try_from(created_at.unix_timestamp_nanos() / 1_000).map_err(|_| {
        DomainNameRepositoryError::Unexpected(format!(
            "kernel.domain_name row {id} was created before the Unix epoch"
        ))
    })?;
    let created_at = DomainNameCreatedAt::from_unix_micros(micros)
        .map_err(|error| DomainNameRepositoryError::Unexpected(error.to_string()))?;

    let domain_name = DomainName::new(stored, created_at);
    if *domain_name.id().value() != id || domain_name.value().to_string() != value {
        return Err(DomainNameRepositoryError::Unexpected(format!(
            "kernel.domain_name row {id} is not stored in canonical form under the id its value derives"
        )));
    }
    Ok(domain_name)
}

fn unexpected(error: sqlx::Error) -> DomainNameRepositoryError {
    DomainNameRepositoryError::Unexpected(error.to_string())
}
