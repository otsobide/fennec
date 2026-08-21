//! Command for deleting a sighting.

use shared_cqrs::command::domain::command::Command;

use crate::sighting::domain::value_objects::sighting_id::SightingId;

/// Command that requests the deletion of a sighting by id.
pub struct DeleteSightingCommand {
    pub id: SightingId,
}

impl Command for DeleteSightingCommand {}
