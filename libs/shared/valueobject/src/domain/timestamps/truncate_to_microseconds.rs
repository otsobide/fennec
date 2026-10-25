//! Microsecond truncation of [`SystemTime`] values.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Drops the sub-microsecond part of an instant, towards the Unix epoch.
///
/// PostgreSQL `timestamptz` stores microseconds, so a timestamp value object
/// that truncates on construction compares equal after a round trip through
/// the database. Truncation is towards the epoch on both sides of it, as the
/// PostgreSQL adapters convert.
///
/// # Example
///
/// ```rust
/// use std::time::{Duration, UNIX_EPOCH};
/// use shared_valueobject::domain::timestamps::truncate_to_microseconds::truncate_to_microseconds;
///
/// let instant = UNIX_EPOCH + Duration::from_nanos(1_700_000_000_123_456_789);
/// assert_eq!(
///     truncate_to_microseconds(instant),
///     UNIX_EPOCH + Duration::from_micros(1_700_000_000_123_456)
/// );
/// ```
pub fn truncate_to_microseconds(value: SystemTime) -> SystemTime {
    match value.duration_since(UNIX_EPOCH) {
        Ok(after) => UNIX_EPOCH + whole_microseconds(after),
        Err(before) => UNIX_EPOCH - whole_microseconds(before.duration()),
    }
}

fn whole_microseconds(duration: Duration) -> Duration {
    Duration::new(duration.as_secs(), duration.subsec_micros() * 1_000)
}
