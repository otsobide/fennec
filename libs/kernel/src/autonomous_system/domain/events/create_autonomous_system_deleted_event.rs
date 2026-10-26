//! Factory function for [`AutonomousSystemDeletedEvent`].

use crate::autonomous_system::domain::entities::autonomous_system::AutonomousSystem;
use crate::autonomous_system::domain::errors::autonomous_system_repository_error::AutonomousSystemRepositoryError;
use crate::autonomous_system::domain::events::autonomous_system_deleted_event::AutonomousSystemDeletedEvent;

/// Creates an [`AutonomousSystemDeletedEvent`] from the deleted autonomous system.
pub fn create_autonomous_system_deleted_event(
    autonomous_system: &AutonomousSystem,
) -> Result<AutonomousSystemDeletedEvent, AutonomousSystemRepositoryError> {
    Ok(AutonomousSystemDeletedEvent::new(autonomous_system))
}
