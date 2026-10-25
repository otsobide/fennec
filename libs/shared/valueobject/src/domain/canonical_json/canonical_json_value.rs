//! JSON value serialised with the JSON Canonicalization Scheme (RFC 8785).

use std::collections::BTreeMap;

use crate::domain::errors::value_object_validation_error::ValueObjectValidationError;

/// Largest integer magnitude that RFC 8785 serialises exactly.
///
/// RFC 8785 serialises numbers as IEEE 754 doubles, which represent every
/// integer up to 2^53 - 1 exactly and nothing reliably beyond it.
pub const MAX_SAFE_INTEGER: i64 = 9_007_199_254_740_991;

/// An immutable JSON value whose serialisation is canonical: the same value
/// always produces the same bytes, as RFC 8785 requires.
///
/// Supports objects, arrays, strings, integers and booleans, which is what
/// STIX ID contributing properties need. Object members are serialised
/// sorted by the UTF-16 code units of their keys, strings use the minimal
/// escaping of ECMAScript `JSON.stringify`, and no whitespace is emitted.
///
/// # Example
///
/// ```rust
/// use shared_valueobject::domain::canonical_json::canonical_json_value::CanonicalJsonValue;
///
/// let value = CanonicalJsonValue::object([
///     ("value", CanonicalJsonValue::string("89.19.209.154")),
/// ]);
/// assert_eq!(value.to_canonical_string(), r#"{"value":"89.19.209.154"}"#);
/// ```
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CanonicalJsonValue(Representation);

#[derive(Clone, PartialEq, Eq, Debug)]
enum Representation {
    Object(BTreeMap<String, CanonicalJsonValue>),
    Array(Vec<CanonicalJsonValue>),
    String(String),
    Integer(i64),
    Boolean(bool),
}

impl CanonicalJsonValue {
    /// Creates a JSON object from its members. A key given twice keeps the
    /// last value.
    pub fn object<K, I>(members: I) -> Self
    where
        K: Into<String>,
        I: IntoIterator<Item = (K, CanonicalJsonValue)>,
    {
        Self(Representation::Object(
            members
                .into_iter()
                .map(|(key, value)| (key.into(), value))
                .collect(),
        ))
    }

    /// Creates a JSON array that keeps the order of its items.
    pub fn array(items: impl IntoIterator<Item = CanonicalJsonValue>) -> Self {
        Self(Representation::Array(items.into_iter().collect()))
    }

    /// Creates a JSON string.
    pub fn string(value: impl Into<String>) -> Self {
        Self(Representation::String(value.into()))
    }

    /// Creates a JSON integer.
    ///
    /// # Errors
    ///
    /// Returns [`ValueObjectValidationError`] when the magnitude of `value`
    /// exceeds [`MAX_SAFE_INTEGER`], because RFC 8785 cannot serialise it
    /// exactly.
    pub fn integer(value: i64) -> Result<Self, ValueObjectValidationError> {
        if value.unsigned_abs() > MAX_SAFE_INTEGER.unsigned_abs() {
            return Err(ValueObjectValidationError::new(format!(
                "canonical JSON integers must be within +/-{MAX_SAFE_INTEGER}, got {value}"
            )));
        }

        Ok(Self(Representation::Integer(value)))
    }

    /// Creates a JSON boolean.
    pub fn boolean(value: bool) -> Self {
        Self(Representation::Boolean(value))
    }

    /// Serialises the value with the JSON Canonicalization Scheme.
    pub fn to_canonical_string(&self) -> String {
        let mut output = String::new();
        self.write_canonical(&mut output);
        output
    }

    fn write_canonical(&self, output: &mut String) {
        match &self.0 {
            Representation::Object(members) => {
                let mut entries: Vec<(&String, &CanonicalJsonValue)> = members.iter().collect();
                entries
                    .sort_by(|(left, _), (right, _)| left.encode_utf16().cmp(right.encode_utf16()));

                output.push('{');
                for (index, (key, value)) in entries.into_iter().enumerate() {
                    if index > 0 {
                        output.push(',');
                    }
                    write_string(key, output);
                    output.push(':');
                    value.write_canonical(output);
                }
                output.push('}');
            }
            Representation::Array(items) => {
                output.push('[');
                for (index, item) in items.iter().enumerate() {
                    if index > 0 {
                        output.push(',');
                    }
                    item.write_canonical(output);
                }
                output.push(']');
            }
            Representation::String(value) => write_string(value, output),
            Representation::Integer(value) => output.push_str(&value.to_string()),
            Representation::Boolean(value) => {
                output.push_str(if *value { "true" } else { "false" })
            }
        }
    }
}

fn write_string(value: &str, output: &mut String) {
    output.push('"');
    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\u{8}' => output.push_str("\\b"),
            '\u{c}' => output.push_str("\\f"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            control if (control as u32) < 0x20 => {
                output.push_str(&format!("\\u{:04x}", control as u32));
            }
            other => output.push(other),
        }
    }
    output.push('"');
}
