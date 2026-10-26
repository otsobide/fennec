//! `AutonomousSystem` aggregate root.
//!
//! Represents one autonomous system that Fennec has registered as an
//! observable. Its identity is its number: two registrations of the same
//! number are the same aggregate. Its name and its registry describe it and
//! can change; they never change its identity.

use crate::autonomous_system::domain::value_objects::autonomous_system_created_at::AutonomousSystemCreatedAt;
use crate::autonomous_system::domain::value_objects::autonomous_system_id::AutonomousSystemId;
use crate::autonomous_system::domain::value_objects::autonomous_system_name::AutonomousSystemName;
use crate::autonomous_system::domain::value_objects::autonomous_system_rir::AutonomousSystemRir;
use crate::autonomous_system::domain::value_objects::autonomous_system_updated_at::AutonomousSystemUpdatedAt;
use crate::autonomous_system::domain::value_objects::autonomous_system_value::AutonomousSystemValue;

/// Aggregate root modelling an observed autonomous system.
///
/// The id is always derived from the number, so an `AutonomousSystem` whose
/// id does not match its number cannot be built. The name and the Regional
/// Internet Registry are optional: a feed often knows only the number.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AutonomousSystem {
    id: AutonomousSystemId,
    value: AutonomousSystemValue,
    name: Option<AutonomousSystemName>,
    rir: Option<AutonomousSystemRir>,
    created_at: AutonomousSystemCreatedAt,
    updated_at: AutonomousSystemUpdatedAt,
}

impl AutonomousSystem {
    /// Creates an `AutonomousSystem` from its number, deriving its id, with
    /// no name and no registry yet; it was last changed when it was created.
    pub fn new(value: AutonomousSystemValue, created_at: AutonomousSystemCreatedAt) -> Self {
        Self {
            id: AutonomousSystemId::from_value(&value),
            value,
            name: None,
            rir: None,
            created_at,
            updated_at: AutonomousSystemUpdatedAt::from_created_at(&created_at),
        }
    }

    /// Rebuilds an `AutonomousSystem` from every stored field, deriving its
    /// id, as a repository reads it back.
    pub fn restore(
        value: AutonomousSystemValue,
        name: Option<AutonomousSystemName>,
        rir: Option<AutonomousSystemRir>,
        created_at: AutonomousSystemCreatedAt,
        updated_at: AutonomousSystemUpdatedAt,
    ) -> Self {
        Self {
            id: AutonomousSystemId::from_value(&value),
            value,
            name,
            rir,
            created_at,
            updated_at,
        }
    }

    /// Returns the same autonomous system with a new name and registry,
    /// changed at `updated_at`. `None` clears a field.
    pub fn describe(
        &self,
        name: Option<AutonomousSystemName>,
        rir: Option<AutonomousSystemRir>,
        updated_at: AutonomousSystemUpdatedAt,
    ) -> Self {
        Self {
            name,
            rir,
            updated_at,
            ..self.clone()
        }
    }

    pub fn id(&self) -> &AutonomousSystemId {
        &self.id
    }

    pub fn value(&self) -> &AutonomousSystemValue {
        &self.value
    }

    pub fn name(&self) -> Option<&AutonomousSystemName> {
        self.name.as_ref()
    }

    pub fn rir(&self) -> Option<&AutonomousSystemRir> {
        self.rir.as_ref()
    }

    pub fn created_at(&self) -> &AutonomousSystemCreatedAt {
        &self.created_at
    }

    pub fn updated_at(&self) -> &AutonomousSystemUpdatedAt {
        &self.updated_at
    }
}
