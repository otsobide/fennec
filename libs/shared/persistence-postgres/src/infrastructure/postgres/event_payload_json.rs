//! JSON rendering of domain event payloads for the event log.

use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::{Map, Value};
use shared_domain_events::domain::primitive_value::PrimitiveValue;
use time::OffsetDateTime;

/// Renders an event payload as the JSON object stored in
/// `event_store.domain_event.payload`.
///
/// Timestamps become RFC 3339 UTC strings with exactly six fractional digits
/// (`2026-10-25T21:00:00.123456Z`), whatever the HTTP API uses for them.
///
/// # Errors
///
/// Returns a message if a timestamp lies outside the range `time` represents.
pub fn event_payload_json(payload: &BTreeMap<String, PrimitiveValue>) -> Result<Value, String> {
    let mut object = Map::new();
    for (key, value) in payload {
        object.insert(key.clone(), primitive_json(value)?);
    }
    Ok(Value::Object(object))
}

/// Converts a [`SystemTime`] to a UTC [`OffsetDateTime`] truncated to
/// microseconds, the precision of `timestamptz`.
///
/// # Errors
///
/// Returns a message if the instant lies outside the range `time` represents.
pub fn to_offset_date_time(value: SystemTime) -> Result<OffsetDateTime, String> {
    let micros: i128 = match value.duration_since(UNIX_EPOCH) {
        Ok(after) => after.as_micros() as i128,
        Err(before) => -(before.duration().as_micros() as i128),
    };

    OffsetDateTime::from_unix_timestamp_nanos(micros * 1_000)
        .map_err(|error| format!("timestamp out of range: {error}"))
}

fn primitive_json(value: &PrimitiveValue) -> Result<Value, String> {
    Ok(match value {
        PrimitiveValue::Null => Value::Null,
        PrimitiveValue::Bool(value) => Value::Bool(*value),
        PrimitiveValue::Integer(value) => Value::from(*value),
        PrimitiveValue::UnsignedInteger(value) => Value::from(*value),
        PrimitiveValue::Text(value) => Value::String(value.clone()),
        PrimitiveValue::Timestamp(value) => Value::String(rfc3339_micros(*value)?),
        PrimitiveValue::List(values) => Value::Array(
            values
                .iter()
                .map(primitive_json)
                .collect::<Result<_, _>>()?,
        ),
        PrimitiveValue::Map(values) => event_payload_json(values)?,
    })
}

fn rfc3339_micros(value: SystemTime) -> Result<String, String> {
    let instant = to_offset_date_time(value)?;

    Ok(format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:06}Z",
        instant.year(),
        u8::from(instant.month()),
        instant.day(),
        instant.hour(),
        instant.minute(),
        instant.second(),
        instant.microsecond()
    ))
}
