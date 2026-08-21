//! [`CommandHandler`] for the update-source use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::source::application::find_source::find_source_response::SourceErrorEntry;
use crate::source::domain::errors::source_repository_error::SourceRepositoryError;

use super::source_updater::SourceUpdater;
use super::update_source_command::UpdateSourceCommand;
use super::update_source_response::UpdateSourceResponse;

/// [`CommandHandler`] that processes [`UpdateSourceCommand`]s by delegating
/// to [`SourceUpdater`].
pub struct UpdateSourceCommandHandler {
    updater: SourceUpdater,
}

impl UpdateSourceCommandHandler {
    pub fn new(updater: SourceUpdater) -> Self {
        Self { updater }
    }
}

#[async_trait]
impl CommandHandler<UpdateSourceCommand> for UpdateSourceCommandHandler {
    type Response = UpdateSourceResponse;

    async fn handle(
        &self,
        command: UpdateSourceCommand,
    ) -> Result<Self::Response, CommandBusError> {
        match self
            .updater
            .execute(command.id, command.status, command.description)
            .await
        {
            Ok(()) => Ok(UpdateSourceResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    SourceRepositoryError::NotFound => "NotFound",
                    SourceRepositoryError::AlreadyExists => "AlreadyExists",
                    SourceRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(UpdateSourceResponse {
                    error: Some(SourceErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
