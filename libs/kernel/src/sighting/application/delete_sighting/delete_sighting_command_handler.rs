//! [`CommandHandler`] for the delete-sighting use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::sighting::application::find_sighting::find_sighting_response::SightingErrorEntry;
use crate::sighting::domain::errors::sighting_repository_error::SightingRepositoryError;

use super::delete_sighting_command::DeleteSightingCommand;
use super::delete_sighting_response::DeleteSightingResponse;
use super::sighting_deleter::SightingDeleter;

/// [`CommandHandler`] that processes [`DeleteSightingCommand`]s by delegating
/// to [`SightingDeleter`].
pub struct DeleteSightingCommandHandler {
    deleter: SightingDeleter,
}

impl DeleteSightingCommandHandler {
    pub fn new(deleter: SightingDeleter) -> Self {
        Self { deleter }
    }
}

#[async_trait]
impl CommandHandler<DeleteSightingCommand> for DeleteSightingCommandHandler {
    type Response = DeleteSightingResponse;

    async fn handle(
        &self,
        command: DeleteSightingCommand,
    ) -> Result<Self::Response, CommandBusError> {
        match self.deleter.execute(command.id).await {
            Ok(()) => Ok(DeleteSightingResponse { error: None }),
            Err(e) => {
                let (concept, existing_id) = match &e {
                    SightingRepositoryError::NotFound => ("NotFound", None),
                    SightingRepositoryError::IdAlreadyExists => ("AlreadyExists", None),
                    SightingRepositoryError::PairAlreadyExists { existing_id } => {
                        ("PairAlreadyExists", Some(existing_id.to_string()))
                    }
                    SightingRepositoryError::Unexpected(_) => ("Unexpected", None),
                };
                Ok(DeleteSightingResponse {
                    error: Some(SightingErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                        existing_id,
                    }),
                })
            }
        }
    }
}
