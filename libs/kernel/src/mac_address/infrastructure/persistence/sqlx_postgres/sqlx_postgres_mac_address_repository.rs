//! PostgreSQL implementation of [`MacAddressRepository`].

use async_trait::async_trait;
use shared_persistence_postgres::infrastructure::postgres::aggregate_lock::AggregateLock;
use shared_persistence_postgres::infrastructure::postgres::pg_session::PgSession;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::mac_address::domain::entities::mac_address::MacAddress;
use crate::mac_address::domain::errors::mac_address_repository_error::MacAddressRepositoryError;
use crate::mac_address::domain::repositories::mac_address_repository::MacAddressRepository;
use crate::mac_address::domain::value_objects::mac_address_created_at::MacAddressCreatedAt;
use crate::mac_address::domain::value_objects::mac_address_id::MacAddressId;
use crate::mac_address::domain::value_objects::mac_address_value::MacAddressValue;

/// The aggregate type of the advisory lock and of the event log rows.
const AGGREGATE_TYPE: &str = "mac_address";

const INSERT: &str = "
    INSERT INTO kernel.mac_address (id, value, created_at)
    VALUES ($1, $2::macaddr, $3)
    ON CONFLICT DO NOTHING";

const SELECT_BY_ID: &str =
    "SELECT id, value::text, created_at FROM kernel.mac_address WHERE id = $1";

const DELETE: &str = "DELETE FROM kernel.mac_address WHERE id = $1";

/// [`MacAddressRepository`] backed by the `kernel.mac_address` table.
///
/// Writes run on the current unit of work after taking the address's
/// advisory lock, so they commit with the command's events. A duplicate is
/// detected with `ON CONFLICT DO NOTHING`, which never aborts the unit of
/// work. Rows are rebuilt through the value objects, and a row whose id is
/// not the one derived from its value is reported as `Unexpected`.
pub struct SqlxPostgresMacAddressRepository {
    session: PgSession,
}

impl SqlxPostgresMacAddressRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            session: PgSession::new(pool),
        }
    }
}

#[async_trait]
impl MacAddressRepository for SqlxPostgresMacAddressRepository {
    async fn save(&self, mac_address: &MacAddress) -> Result<(), MacAddressRepositoryError> {
        let id = *mac_address.id().value();
        let value = mac_address.value().to_string();
        let created_at = to_offset_date_time(mac_address.created_at())?;

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
            return Err(MacAddressRepositoryError::AlreadyExists);
        }
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &MacAddressId,
    ) -> Result<Option<MacAddress>, MacAddressRepositoryError> {
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

    async fn delete(&self, id: &MacAddressId) -> Result<(), MacAddressRepositoryError> {
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
            return Err(MacAddressRepositoryError::NotFound);
        }
        Ok(())
    }
}

fn lock(id: &Uuid) -> AggregateLock {
    AggregateLock::new(AGGREGATE_TYPE, id)
}

fn to_offset_date_time(
    created_at: &MacAddressCreatedAt,
) -> Result<OffsetDateTime, MacAddressRepositoryError> {
    OffsetDateTime::from_unix_timestamp_nanos(i128::from(created_at.unix_micros()) * 1_000)
        .map_err(|error| MacAddressRepositoryError::Unexpected(error.to_string()))
}

fn to_aggregate(
    id: Uuid,
    value: String,
    created_at: OffsetDateTime,
) -> Result<MacAddress, MacAddressRepositoryError> {
    let stored = MacAddressValue::new(&value).map_err(|error| {
        MacAddressRepositoryError::Unexpected(format!(
            "kernel.mac_address row {id} holds an invalid value: {error}"
        ))
    })?;
    let micros = u64::try_from(created_at.unix_timestamp_nanos() / 1_000).map_err(|_| {
        MacAddressRepositoryError::Unexpected(format!(
            "kernel.mac_address row {id} was created before the Unix epoch"
        ))
    })?;
    let created_at = MacAddressCreatedAt::from_unix_micros(micros)
        .map_err(|error| MacAddressRepositoryError::Unexpected(error.to_string()))?;

    let mac_address = MacAddress::new(stored, created_at);
    if *mac_address.id().value() != id || mac_address.value().to_string() != value {
        return Err(MacAddressRepositoryError::Unexpected(format!(
            "kernel.mac_address row {id} is not stored in canonical form under the id its value derives"
        )));
    }
    Ok(mac_address)
}

fn unexpected(error: sqlx::Error) -> MacAddressRepositoryError {
    MacAddressRepositoryError::Unexpected(error.to_string())
}
