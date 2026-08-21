//! [`CommandHandler`] for the delete-source use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::source::application::find_source::find_source_response::SourceErrorEntry;
use crate::source::domain::errors::source_repository_error::SourceRepositoryError;

use super::delete_source_command::DeleteSourceCommand;
use super::delete_source_response::DeleteSourceResponse;
use super::source_deleter::SourceDeleter;

/// [`CommandHandler`] that processes [`DeleteSourceCommand`]s by delegating
/// to [`SourceDeleter`].
pub struct DeleteSourceCommandHandler {
    deleter: SourceDeleter,
}

impl DeleteSourceCommandHandler {
    pub fn new(deleter: SourceDeleter) -> Self {
        Self { deleter }
    }
}

#[async_trait]
impl CommandHandler<DeleteSourceCommand> for DeleteSourceCommandHandler {
    type Response = DeleteSourceResponse;

    async fn handle(
        &self,
        command: DeleteSourceCommand,
    ) -> Result<Self::Response, CommandBusError> {
        match self.deleter.execute(command.id).await {
            Ok(()) => Ok(DeleteSourceResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    SourceRepositoryError::NotFound => "NotFound",
                    SourceRepositoryError::AlreadyExists => "AlreadyExists",
                    SourceRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(DeleteSourceResponse {
                    error: Some(SourceErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
