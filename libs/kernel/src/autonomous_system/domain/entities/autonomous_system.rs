//! `AutonomousSystem` aggregate root.
//!
//! Represents one autonomous system that Fennec has registered as an
//! observable. Its identity is its value: two registrations of the same
//! autonomous system are the same aggregate.

use crate::autonomous_system::domain::value_objects::autonomous_system_created_at::AutonomousSystemCreatedAt;
use crate::autonomous_system::domain::value_objects::autonomous_system_id::AutonomousSystemId;
use crate::autonomous_system::domain::value_objects::autonomous_system_value::AutonomousSystemValue;

/// Aggregate root modelling an observed autonomous system.
///
/// The id is always derived from the value, so an `AutonomousSystem` whose id does
/// not match its autonomous system cannot be built.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AutonomousSystem {
    id: AutonomousSystemId,
    value: AutonomousSystemValue,
    created_at: AutonomousSystemCreatedAt,
}

impl AutonomousSystem {
    /// Creates an `AutonomousSystem` from its value, deriving its id.
    pub fn new(value: AutonomousSystemValue, created_at: AutonomousSystemCreatedAt) -> Self {
        Self {
            id: AutonomousSystemId::from_value(&value),
            value,
            created_at,
        }
    }

    pub fn id(&self) -> &AutonomousSystemId {
        &self.id
    }

    pub fn value(&self) -> &AutonomousSystemValue {
        &self.value
    }

    pub fn created_at(&self) -> &AutonomousSystemCreatedAt {
        &self.created_at
    }
}
