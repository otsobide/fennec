//! Writes domain events to the event log.

use shared_domain_events::domain::domain_event::DomainEvent;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::infrastructure::postgres::event_payload_json::{
    event_payload_json, to_offset_date_time,
};

/// Appends events, in order, with one statement.
const APPEND_EVENTS: &str = "
    INSERT INTO event_store.domain_event
        (occurred_on, event_id, event_name, aggregate_type, aggregate_id, payload)
    SELECT occurred_on, event_id, event_name, aggregate_type, aggregate_id, payload::jsonb
    FROM UNNEST($1::timestamptz[], $2::uuid[], $3::text[], $4::text[], $5::uuid[], $6::text[])
        WITH ORDINALITY
        AS event(occurred_on, event_id, event_name, aggregate_type, aggregate_id, payload, ordinal)
    ORDER BY ordinal";

/// The writer of `event_store.domain_event`.
///
/// The aggregate type stored next to each event is the `<aggregate>` segment
/// of its `fennec.<context>.<aggregate>.<past>` name, which is the module
/// kind, because ids of different kinds can share a UUID.
pub struct PgEventStore;

impl PgEventStore {
    /// Appends `events` on `connection`, which is the transaction of the
    /// state change they describe.
    ///
    /// # Errors
    ///
    /// Returns [`sqlx::Error::Encode`] if an event's ids are not UUIDs, its
    /// name has no aggregate segment or a timestamp is out of range, and the
    /// driver error if the insert fails.
    pub async fn append(
        connection: &mut PgConnection,
        events: &[Box<dyn DomainEvent>],
    ) -> Result<(), sqlx::Error> {
        if events.is_empty() {
            return Ok(());
        }

        let mut occurred_on = Vec::with_capacity(events.len());
        let mut event_ids = Vec::with_capacity(events.len());
        let mut event_names = Vec::with_capacity(events.len());
        let mut aggregate_types = Vec::with_capacity(events.len());
        let mut aggregate_ids = Vec::with_capacity(events.len());
        let mut payloads = Vec::with_capacity(events.len());

        for event in events {
            occurred_on.push(to_offset_date_time(event.occurred_on()).map_err(encode_error)?);
            event_ids.push(parse_uuid("event id", event.event_id())?);
            event_names.push(event.event_name().to_string());
            aggregate_types.push(aggregate_type_of(event.event_name())?);
            aggregate_ids.push(parse_uuid("aggregate id", event.aggregate_id())?);
            payloads.push(
                event_payload_json(&event.to_primitives())
                    .map_err(encode_error)?
                    .to_string(),
            );
        }

        sqlx::query(APPEND_EVENTS)
            .bind(occurred_on)
            .bind(event_ids)
            .bind(event_names)
            .bind(aggregate_types)
            .bind(aggregate_ids)
            .bind(payloads)
            .execute(connection)
            .await
            .map(|_| ())
    }
}

fn parse_uuid(field: &str, value: &str) -> Result<Uuid, sqlx::Error> {
    Uuid::parse_str(value).map_err(|_| {
        encode_error(format!(
            "the {field} of an event must be a UUID, got {value}"
        ))
    })
}

fn aggregate_type_of(event_name: &str) -> Result<String, sqlx::Error> {
    event_name
        .split('.')
        .nth(2)
        .filter(|segment| !segment.is_empty())
        .map(str::to_string)
        .ok_or_else(|| {
            encode_error(format!(
                "event names must be fennec.<context>.<aggregate>.<past>, got {event_name}"
            ))
        })
}

fn encode_error(message: String) -> sqlx::Error {
    sqlx::Error::Encode(message.into())
}
