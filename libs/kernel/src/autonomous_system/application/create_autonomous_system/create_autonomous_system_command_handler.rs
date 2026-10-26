//! [`CommandHandler`] for the create-autonomous-system use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::autonomous_system::application::find_autonomous_system::find_autonomous_system_response::AutonomousSystemErrorEntry;
use crate::autonomous_system::domain::errors::autonomous_system_repository_error::AutonomousSystemRepositoryError;

use super::autonomous_system_creator::AutonomousSystemCreator;
use super::create_autonomous_system_command::CreateAutonomousSystemCommand;
use super::create_autonomous_system_response::CreateAutonomousSystemResponse;

/// [`CommandHandler`] that processes [`CreateAutonomousSystemCommand`]s by
/// delegating to [`AutonomousSystemCreator`].
pub struct CreateAutonomousSystemCommandHandler {
    creator: AutonomousSystemCreator,
}

impl CreateAutonomousSystemCommandHandler {
    pub fn new(creator: AutonomousSystemCreator) -> Self {
        Self { creator }
    }
}

#[async_trait]
impl CommandHandler<CreateAutonomousSystemCommand> for CreateAutonomousSystemCommandHandler {
    type Response = CreateAutonomousSystemResponse;

    async fn handle(
        &self,
        command: CreateAutonomousSystemCommand,
    ) -> Result<Self::Response, CommandBusError> {
        match self.creator.execute(command.value).await {
            Ok(()) => Ok(CreateAutonomousSystemResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    AutonomousSystemRepositoryError::NotFound => "NotFound",
                    AutonomousSystemRepositoryError::AlreadyExists => "AlreadyExists",
                    AutonomousSystemRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(CreateAutonomousSystemResponse {
                    error: Some(AutonomousSystemErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
