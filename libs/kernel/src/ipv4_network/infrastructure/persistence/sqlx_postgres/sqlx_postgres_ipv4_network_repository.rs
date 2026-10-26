//! PostgreSQL implementation of [`Ipv4NetworkRepository`].

use async_trait::async_trait;
use shared_persistence_postgres::infrastructure::postgres::aggregate_lock::AggregateLock;
use shared_persistence_postgres::infrastructure::postgres::pg_session::PgSession;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::ipv4_network::domain::entities::ipv4_network::Ipv4Network;
use crate::ipv4_network::domain::errors::ipv4_network_repository_error::Ipv4NetworkRepositoryError;
use crate::ipv4_network::domain::repositories::ipv4_network_repository::Ipv4NetworkRepository;
use crate::ipv4_network::domain::value_objects::ipv4_network_created_at::Ipv4NetworkCreatedAt;
use crate::ipv4_network::domain::value_objects::ipv4_network_id::Ipv4NetworkId;
use crate::ipv4_network::domain::value_objects::ipv4_network_value::Ipv4NetworkValue;

/// The aggregate type of the advisory lock and of the event log rows.
const AGGREGATE_TYPE: &str = "ipv4_network";

const INSERT: &str = "
    INSERT INTO kernel.ipv4_network (id, value, created_at)
    VALUES ($1, $2::cidr, $3)
    ON CONFLICT DO NOTHING";

const SELECT_BY_ID: &str =
    "SELECT id, value::text, created_at FROM kernel.ipv4_network WHERE id = $1";

const DELETE: &str = "DELETE FROM kernel.ipv4_network WHERE id = $1";

/// [`Ipv4NetworkRepository`] backed by the `kernel.ipv4_network` table.
///
/// Writes run on the current unit of work after taking the network's
/// advisory lock, so they commit with the command's events. A duplicate is
/// detected with `ON CONFLICT DO NOTHING`, which never aborts the unit of
/// work. Rows are rebuilt through the value objects, and a row whose id is
/// not the one derived from its value is reported as `Unexpected`.
pub struct SqlxPostgresIpv4NetworkRepository {
    session: PgSession,
}

impl SqlxPostgresIpv4NetworkRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            session: PgSession::new(pool),
        }
    }
}

#[async_trait]
impl Ipv4NetworkRepository for SqlxPostgresIpv4NetworkRepository {
    async fn save(&self, ipv4_network: &Ipv4Network) -> Result<(), Ipv4NetworkRepositoryError> {
        let id = *ipv4_network.id().value();
        let value = ipv4_network.value().to_string();
        let created_at = to_offset_date_time(ipv4_network.created_at())?;

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
            return Err(Ipv4NetworkRepositoryError::AlreadyExists);
        }
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &Ipv4NetworkId,
    ) -> Result<Option<Ipv4Network>, Ipv4NetworkRepositoryError> {
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

    async fn delete(&self, id: &Ipv4NetworkId) -> Result<(), Ipv4NetworkRepositoryError> {
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
            return Err(Ipv4NetworkRepositoryError::NotFound);
        }
        Ok(())
    }
}

fn lock(id: &Uuid) -> AggregateLock {
    AggregateLock::new(AGGREGATE_TYPE, id)
}

fn to_offset_date_time(
    created_at: &Ipv4NetworkCreatedAt,
) -> Result<OffsetDateTime, Ipv4NetworkRepositoryError> {
    OffsetDateTime::from_unix_timestamp_nanos(i128::from(created_at.unix_micros()) * 1_000)
        .map_err(|error| Ipv4NetworkRepositoryError::Unexpected(error.to_string()))
}

fn to_aggregate(
    id: Uuid,
    value: String,
    created_at: OffsetDateTime,
) -> Result<Ipv4Network, Ipv4NetworkRepositoryError> {
    let stored = Ipv4NetworkValue::new(&value).map_err(|error| {
        Ipv4NetworkRepositoryError::Unexpected(format!(
            "kernel.ipv4_network row {id} holds an invalid value: {error}"
        ))
    })?;
    let micros = u64::try_from(created_at.unix_timestamp_nanos() / 1_000).map_err(|_| {
        Ipv4NetworkRepositoryError::Unexpected(format!(
            "kernel.ipv4_network row {id} was created before the Unix epoch"
        ))
    })?;
    let created_at = Ipv4NetworkCreatedAt::from_unix_micros(micros)
        .map_err(|error| Ipv4NetworkRepositoryError::Unexpected(error.to_string()))?;

    let ipv4_network = Ipv4Network::new(stored, created_at);
    if *ipv4_network.id().value() != id || ipv4_network.value().to_string() != value {
        return Err(Ipv4NetworkRepositoryError::Unexpected(format!(
            "kernel.ipv4_network row {id} is not stored in canonical form under the id its value derives"
        )));
    }
    Ok(ipv4_network)
}

fn unexpected(error: sqlx::Error) -> Ipv4NetworkRepositoryError {
    Ipv4NetworkRepositoryError::Unexpected(error.to_string())
}
