//! Serialisation-free representation of a domain event payload.

use std::collections::BTreeMap;
use std::time::SystemTime;

/// A plain data value inside a domain event payload.
///
/// Events describe their payload with these values so that the domain never
/// depends on a serialisation crate. Infrastructure adapters (the PostgreSQL
/// event log, a message relay) translate them into their own format.
///
/// # Example
///
/// ```rust
/// use std::collections::BTreeMap;
/// use shared_domain_events::domain::primitive_value::PrimitiveValue;
///
/// let payload = BTreeMap::from([
///     ("id".to_string(), PrimitiveValue::Text("abfb438a-8baf-5f54-b19e-b85ee74b3ed7".to_string())),
///     ("count".to_string(), PrimitiveValue::UnsignedInteger(1)),
/// ]);
/// assert_eq!(payload["count"], PrimitiveValue::UnsignedInteger(1));
/// ```
#[derive(Clone, PartialEq, Debug)]
pub enum PrimitiveValue {
    /// The absence of a value, for optional fields.
    Null,
    /// A boolean.
    Bool(bool),
    /// A signed integer.
    Integer(i64),
    /// An unsigned integer (counts, unsigned identifiers).
    UnsignedInteger(u64),
    /// A string: identifiers, canonical values and enum names.
    Text(String),
    /// An instant.
    Timestamp(SystemTime),
    /// An ordered list of values.
    List(Vec<PrimitiveValue>),
    /// A map of named values.
    Map(BTreeMap<String, PrimitiveValue>),
}
