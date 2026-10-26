//! [`CommandHandler`] for the update-autonomous-system use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::autonomous_system::application::find_autonomous_system::find_autonomous_system_response::AutonomousSystemErrorEntry;
use crate::autonomous_system::domain::errors::autonomous_system_repository_error::AutonomousSystemRepositoryError;

use super::autonomous_system_updater::AutonomousSystemUpdater;
use super::update_autonomous_system_command::UpdateAutonomousSystemCommand;
use super::update_autonomous_system_response::UpdateAutonomousSystemResponse;

/// [`CommandHandler`] that processes [`UpdateAutonomousSystemCommand`]s by
/// delegating to [`AutonomousSystemUpdater`].
pub struct UpdateAutonomousSystemCommandHandler {
    updater: AutonomousSystemUpdater,
}

impl UpdateAutonomousSystemCommandHandler {
    pub fn new(updater: AutonomousSystemUpdater) -> Self {
        Self { updater }
    }
}

#[async_trait]
impl CommandHandler<UpdateAutonomousSystemCommand> for UpdateAutonomousSystemCommandHandler {
    type Response = UpdateAutonomousSystemResponse;

    async fn handle(
        &self,
        command: UpdateAutonomousSystemCommand,
    ) -> Result<Self::Response, CommandBusError> {
        match self
            .updater
            .execute(command.id, command.name, command.rir)
            .await
        {
            Ok(()) => Ok(UpdateAutonomousSystemResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    AutonomousSystemRepositoryError::NotFound => "NotFound",
                    AutonomousSystemRepositoryError::AlreadyExists => "AlreadyExists",
                    AutonomousSystemRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(UpdateAutonomousSystemResponse {
                    error: Some(AutonomousSystemErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
