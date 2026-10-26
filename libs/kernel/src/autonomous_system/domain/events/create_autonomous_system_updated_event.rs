//! Factory function for [`AutonomousSystemUpdatedEvent`].

use crate::autonomous_system::domain::entities::autonomous_system::AutonomousSystem;
use crate::autonomous_system::domain::errors::autonomous_system_repository_error::AutonomousSystemRepositoryError;
use crate::autonomous_system::domain::events::autonomous_system_updated_event::AutonomousSystemUpdatedEvent;

/// Creates an [`AutonomousSystemUpdatedEvent`] from the autonomous system
/// after and before the change.
pub fn create_autonomous_system_updated_event(
    updated: &AutonomousSystem,
    previous: &AutonomousSystem,
) -> Result<AutonomousSystemUpdatedEvent, AutonomousSystemRepositoryError> {
    Ok(AutonomousSystemUpdatedEvent::new(updated, previous))
}
