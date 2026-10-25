//! [`CommandHandler`] for the create-source use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::source::application::find_source::find_source_response::SourceErrorEntry;
use crate::source::domain::errors::source_repository_error::SourceRepositoryError;

use super::create_source_command::CreateSourceCommand;
use super::create_source_response::CreateSourceResponse;
use super::source_creator::SourceCreator;

/// [`CommandHandler`] that processes [`CreateSourceCommand`]s by delegating
/// to [`SourceCreator`].
pub struct CreateSourceCommandHandler {
    creator: SourceCreator,
}

impl CreateSourceCommandHandler {
    pub fn new(creator: SourceCreator) -> Self {
        Self { creator }
    }
}

#[async_trait]
impl CommandHandler<CreateSourceCommand> for CreateSourceCommandHandler {
    type Response = CreateSourceResponse;

    async fn handle(
        &self,
        command: CreateSourceCommand,
    ) -> Result<Self::Response, CommandBusError> {
        match self
            .creator
            .execute(
                command.id,
                command.source_type,
                command.status,
                command.description,
            )
            .await
        {
            Ok(()) => Ok(CreateSourceResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    SourceRepositoryError::NotFound => "NotFound",
                    SourceRepositoryError::AlreadyExists => "AlreadyExists",
                    SourceRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(CreateSourceResponse {
                    error: Some(SourceErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
