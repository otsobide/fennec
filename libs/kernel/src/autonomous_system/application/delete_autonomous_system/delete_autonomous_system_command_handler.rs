//! [`CommandHandler`] for the delete-autonomous-system use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::autonomous_system::application::find_autonomous_system::find_autonomous_system_response::AutonomousSystemErrorEntry;
use crate::autonomous_system::domain::errors::autonomous_system_repository_error::AutonomousSystemRepositoryError;

use super::autonomous_system_deleter::AutonomousSystemDeleter;
use super::delete_autonomous_system_command::DeleteAutonomousSystemCommand;
use super::delete_autonomous_system_response::DeleteAutonomousSystemResponse;

/// [`CommandHandler`] that processes [`DeleteAutonomousSystemCommand`]s by
/// delegating to [`AutonomousSystemDeleter`].
pub struct DeleteAutonomousSystemCommandHandler {
    deleter: AutonomousSystemDeleter,
}

impl DeleteAutonomousSystemCommandHandler {
    pub fn new(deleter: AutonomousSystemDeleter) -> Self {
        Self { deleter }
    }
}

#[async_trait]
impl CommandHandler<DeleteAutonomousSystemCommand> for DeleteAutonomousSystemCommandHandler {
    type Response = DeleteAutonomousSystemResponse;

    async fn handle(
        &self,
        command: DeleteAutonomousSystemCommand,
    ) -> Result<Self::Response, CommandBusError> {
        match self.deleter.execute(command.id).await {
            Ok(()) => Ok(DeleteAutonomousSystemResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    AutonomousSystemRepositoryError::NotFound => "NotFound",
                    AutonomousSystemRepositoryError::AlreadyExists => "AlreadyExists",
                    AutonomousSystemRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(DeleteAutonomousSystemResponse {
                    error: Some(AutonomousSystemErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
