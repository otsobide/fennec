//! Factory function for [`AutonomousSystemCreatedEvent`].

use crate::autonomous_system::domain::entities::autonomous_system::AutonomousSystem;
use crate::autonomous_system::domain::errors::autonomous_system_repository_error::AutonomousSystemRepositoryError;
use crate::autonomous_system::domain::events::autonomous_system_created_event::AutonomousSystemCreatedEvent;

/// Creates an [`AutonomousSystemCreatedEvent`] from the given autonomous system.
pub fn create_autonomous_system_created_event(
    autonomous_system: &AutonomousSystem,
) -> Result<AutonomousSystemCreatedEvent, AutonomousSystemRepositoryError> {
    Ok(AutonomousSystemCreatedEvent::new(autonomous_system))
}
