//! PostgreSQL implementation of [`SightingRepository`].

use std::time::SystemTime;

use async_trait::async_trait;
use shared_persistence_postgres::infrastructure::postgres::aggregate_lock::AggregateLock;
use shared_persistence_postgres::infrastructure::postgres::event_payload_json::to_offset_date_time;
use shared_persistence_postgres::infrastructure::postgres::pg_session::PgSession;
use shared_valueobject::domain::errors::value_object_validation_error::ValueObjectValidationError;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::sighting::domain::entities::sighting::Sighting;
use crate::sighting::domain::errors::sighting_repository_error::SightingRepositoryError;
use crate::sighting::domain::repositories::sighting_repository::SightingRepository;
use crate::sighting::domain::value_objects::sighting_count::SightingCount;
use crate::sighting::domain::value_objects::sighting_created_at::SightingCreatedAt;
use crate::sighting::domain::value_objects::sighting_first_seen::SightingFirstSeen;
use crate::sighting::domain::value_objects::sighting_id::SightingId;
use crate::sighting::domain::value_objects::sighting_last_seen::SightingLastSeen;
use crate::sighting::domain::value_objects::sighting_observable_id::SightingObservableId;
use crate::sighting::domain::value_objects::sighting_observable_kind::SightingObservableKind;
use crate::sighting::domain::value_objects::sighting_source_id::SightingSourceId;
use crate::sighting::domain::value_objects::sighting_updated_at::SightingUpdatedAt;

/// The aggregate type of the advisory lock and of the event log rows.
const AGGREGATE_TYPE: &str = "sighting";

const COLUMNS: &str = "id, observable_kind, observable_id, source_id, first_seen, last_seen, count, created_at, updated_at";

const INSERT: &str = "
    INSERT INTO kernel.sighting
        (id, observable_kind, observable_id, source_id, first_seen, last_seen, count, created_at, updated_at)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
    ON CONFLICT DO NOTHING";

/// Which existing row a rejected insert collided with: the triple wins over
/// the id, as in the in-memory repository, so the caller learns the id of
/// the sighting to observe.
const SELECT_CONFLICT: &str = "
    SELECT id, (observable_kind = $2 AND observable_id = $3 AND source_id = $4) AS same_triple
    FROM kernel.sighting
    WHERE id = $1 OR (observable_kind = $2 AND observable_id = $3 AND source_id = $4)
    ORDER BY same_triple DESC
    LIMIT 1";

const UPDATE: &str = "
    UPDATE kernel.sighting
    SET first_seen = $2, last_seen = $3, count = $4, updated_at = $5
    WHERE id = $1";

const DELETE: &str = "DELETE FROM kernel.sighting WHERE id = $1";

type SightingRow = (
    Uuid,
    String,
    Uuid,
    Uuid,
    OffsetDateTime,
    OffsetDateTime,
    i64,
    OffsetDateTime,
    OffsetDateTime,
);

/// The outcome of an insert, decided inside one database round trip.
enum SaveOutcome {
    Inserted,
    TripleTaken(Uuid),
    IdTaken,
    ConflictVanished,
}

/// [`SightingRepository`] backed by the `kernel.sighting` table.
///
/// Writes run on the current unit of work after taking the sighting's
/// advisory lock, so an observation reads and writes the committed counter
/// and no concurrent observation is lost. Duplicates are detected with
/// `ON CONFLICT DO NOTHING`, which never aborts the unit of work, and then
/// classified as a taken (kind, id, source) triple or a taken id.
pub struct SqlxPostgresSightingRepository {
    session: PgSession,
}

impl SqlxPostgresSightingRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            session: PgSession::new(pool),
        }
    }

    async fn select_many(
        &self,
        filter: &'static str,
        first: Uuid,
        kind: Option<String>,
    ) -> Result<Vec<Sighting>, SightingRepositoryError> {
        let rows = self
            .session
            .run_unlocked(move |connection| {
                Box::pin(async move {
                    let sql = format!(
                        "SELECT {COLUMNS} FROM kernel.sighting WHERE {filter} ORDER BY created_at, id"
                    );
                    let query = sqlx::query_as::<_, SightingRow>(sqlx::AssertSqlSafe(sql));
                    match kind {
                        Some(kind) => query.bind(kind).bind(first),
                        None => query.bind(first),
                    }
                    .fetch_all(connection)
                    .await
                })
            })
            .await
            .map_err(unexpected)?;

        rows.into_iter().map(to_aggregate).collect()
    }
}

#[async_trait]
impl SightingRepository for SqlxPostgresSightingRepository {
    async fn save(&self, sighting: &Sighting) -> Result<(), SightingRepositoryError> {
        let id = *sighting.id().value();
        let observable_kind = sighting.observable_kind().value().to_string();
        let observable_id = *sighting.observable_id().value();
        let source_id = *sighting.source_id().value();
        let first_seen = timestamp(sighting.first_seen().value())?;
        let last_seen = timestamp(sighting.last_seen().value())?;
        let count = count(sighting.count())?;
        let created_at = timestamp(sighting.created_at().value())?;
        let updated_at = timestamp(sighting.updated_at().value())?;

        let outcome = self
            .session
            .run(lock(&id), move |connection| {
                Box::pin(async move {
                    let inserted = sqlx::query(INSERT)
                        .bind(id)
                        .bind(&observable_kind)
                        .bind(observable_id)
                        .bind(source_id)
                        .bind(first_seen)
                        .bind(last_seen)
                        .bind(count)
                        .bind(created_at)
                        .bind(updated_at)
                        .execute(&mut *connection)
                        .await?
                        .rows_affected();
                    if inserted == 1 {
                        return Ok(SaveOutcome::Inserted);
                    }

                    let conflict = sqlx::query_as::<_, (Uuid, bool)>(SELECT_CONFLICT)
                        .bind(id)
                        .bind(&observable_kind)
                        .bind(observable_id)
                        .bind(source_id)
                        .fetch_optional(connection)
                        .await?;
                    Ok(match conflict {
                        Some((existing_id, true)) => SaveOutcome::TripleTaken(existing_id),
                        Some((_, false)) => SaveOutcome::IdTaken,
                        None => SaveOutcome::ConflictVanished,
                    })
                })
            })
            .await
            .map_err(unexpected)?;

        match outcome {
            SaveOutcome::Inserted => Ok(()),
            SaveOutcome::TripleTaken(existing_id) => {
                Err(SightingRepositoryError::PairAlreadyExists {
                    existing_id: SightingId::from_uuid(existing_id).map_err(|error| {
                        SightingRepositoryError::Unexpected(format!(
                            "kernel.sighting row {existing_id} is invalid: {error}"
                        ))
                    })?,
                })
            }
            SaveOutcome::IdTaken => Err(SightingRepositoryError::IdAlreadyExists),
            SaveOutcome::ConflictVanished => Err(SightingRepositoryError::Unexpected(
                "the sighting an insert collided with was deleted concurrently".to_string(),
            )),
        }
    }

    async fn update(&self, sighting: &Sighting) -> Result<(), SightingRepositoryError> {
        let id = *sighting.id().value();
        let first_seen = timestamp(sighting.first_seen().value())?;
        let last_seen = timestamp(sighting.last_seen().value())?;
        let count = count(sighting.count())?;
        let updated_at = timestamp(sighting.updated_at().value())?;

        let updated = self
            .session
            .run(lock(&id), move |connection| {
                Box::pin(async move {
                    sqlx::query(UPDATE)
                        .bind(id)
                        .bind(first_seen)
                        .bind(last_seen)
                        .bind(count)
                        .bind(updated_at)
                        .execute(connection)
                        .await
                        .map(|result| result.rows_affected())
                })
            })
            .await
            .map_err(unexpected)?;

        if updated == 0 {
            return Err(SightingRepositoryError::NotFound);
        }
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &SightingId,
    ) -> Result<Option<Sighting>, SightingRepositoryError> {
        let id = *id.value();

        let row = self
            .session
            .run(lock(&id), move |connection| {
                Box::pin(async move {
                    sqlx::query_as::<_, SightingRow>(sqlx::AssertSqlSafe(format!(
                        "SELECT {COLUMNS} FROM kernel.sighting WHERE id = $1"
                    )))
                    .bind(id)
                    .fetch_optional(connection)
                    .await
                })
            })
            .await
            .map_err(unexpected)?;

        row.map(to_aggregate).transpose()
    }

    async fn find_by_observable_and_source(
        &self,
        observable_kind: &SightingObservableKind,
        observable_id: &SightingObservableId,
        source_id: &SightingSourceId,
    ) -> Result<Option<Sighting>, SightingRepositoryError> {
        let observable_kind = observable_kind.value().to_string();
        let observable_id = *observable_id.value();
        let source_id = *source_id.value();

        let row = self
            .session
            .run_unlocked(move |connection| {
                Box::pin(async move {
                    sqlx::query_as::<_, SightingRow>(sqlx::AssertSqlSafe(format!(
                        "SELECT {COLUMNS} FROM kernel.sighting
                         WHERE observable_kind = $1 AND observable_id = $2 AND source_id = $3"
                    )))
                    .bind(observable_kind)
                    .bind(observable_id)
                    .bind(source_id)
                    .fetch_optional(connection)
                    .await
                })
            })
            .await
            .map_err(unexpected)?;

        row.map(to_aggregate).transpose()
    }

    async fn find_by_observable(
        &self,
        observable_kind: &SightingObservableKind,
        observable_id: &SightingObservableId,
    ) -> Result<Vec<Sighting>, SightingRepositoryError> {
        self.select_many(
            "observable_kind = $1 AND observable_id = $2",
            *observable_id.value(),
            Some(observable_kind.value().to_string()),
        )
        .await
    }

    async fn find_by_source_id(
        &self,
        source_id: &SightingSourceId,
    ) -> Result<Vec<Sighting>, SightingRepositoryError> {
        self.select_many("source_id = $1", *source_id.value(), None)
            .await
    }

    async fn delete(&self, id: &SightingId) -> Result<(), SightingRepositoryError> {
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
            return Err(SightingRepositoryError::NotFound);
        }
        Ok(())
    }
}

fn lock(id: &Uuid) -> AggregateLock {
    AggregateLock::new(AGGREGATE_TYPE, id)
}

fn timestamp(value: SystemTime) -> Result<OffsetDateTime, SightingRepositoryError> {
    to_offset_date_time(value).map_err(SightingRepositoryError::Unexpected)
}

fn count(count: &SightingCount) -> Result<i64, SightingRepositoryError> {
    i64::try_from(count.value()).map_err(|_| {
        SightingRepositoryError::Unexpected(format!(
            "sighting count {} does not fit in a bigint",
            count.value()
        ))
    })
}

fn to_aggregate(row: SightingRow) -> Result<Sighting, SightingRepositoryError> {
    let (
        id,
        observable_kind,
        observable_id,
        source_id,
        first_seen,
        last_seen,
        count,
        created_at,
        updated_at,
    ) = row;
    let invalid = |error: ValueObjectValidationError| {
        SightingRepositoryError::Unexpected(format!("kernel.sighting row {id} is invalid: {error}"))
    };
    let count = u64::try_from(count).map_err(|_| {
        SightingRepositoryError::Unexpected(format!(
            "kernel.sighting row {id} has the negative count {count}"
        ))
    })?;

    Ok(Sighting::new(
        SightingId::from_uuid(id).map_err(invalid)?,
        SightingObservableKind::new(&observable_kind).map_err(invalid)?,
        SightingObservableId::from_uuid(observable_id).map_err(invalid)?,
        SightingSourceId::from_uuid(source_id).map_err(invalid)?,
        SightingFirstSeen::from_system_time(first_seen.into()),
        SightingLastSeen::from_system_time(last_seen.into()),
        SightingCount::from_u64(count).map_err(invalid)?,
        SightingCreatedAt::from_system_time(created_at.into()),
        SightingUpdatedAt::from_system_time(updated_at.into()),
    ))
}

fn unexpected(error: sqlx::Error) -> SightingRepositoryError {
    SightingRepositoryError::Unexpected(error.to_string())
}
