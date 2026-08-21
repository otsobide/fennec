//! [`CommandHandler`] for the create-sighting use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::sighting::application::find_sighting::find_sighting_response::SightingErrorEntry;
use crate::sighting::domain::errors::sighting_repository_error::SightingRepositoryError;

use super::create_sighting_command::CreateSightingCommand;
use super::create_sighting_response::CreateSightingResponse;
use super::sighting_creator::SightingCreator;

/// [`CommandHandler`] that processes [`CreateSightingCommand`]s by delegating
/// to [`SightingCreator`].
pub struct CreateSightingCommandHandler {
    creator: SightingCreator,
}

impl CreateSightingCommandHandler {
    pub fn new(creator: SightingCreator) -> Self {
        Self { creator }
    }
}

#[async_trait]
impl CommandHandler<CreateSightingCommand> for CreateSightingCommandHandler {
    type Response = CreateSightingResponse;

    async fn handle(
        &self,
        command: CreateSightingCommand,
    ) -> Result<Self::Response, CommandBusError> {
        match self
            .creator
            .execute(
                command.id,
                command.ioc_id,
                command.source_id,
                command.observed_at,
            )
            .await
        {
            Ok(()) => Ok(CreateSightingResponse { error: None }),
            Err(e) => {
                let (concept, existing_id) = match &e {
                    SightingRepositoryError::NotFound => ("NotFound", None),
                    SightingRepositoryError::IdAlreadyExists => ("AlreadyExists", None),
                    SightingRepositoryError::PairAlreadyExists { existing_id } => {
                        ("PairAlreadyExists", Some(existing_id.to_string()))
                    }
                    SightingRepositoryError::Unexpected(_) => ("Unexpected", None),
                };
                Ok(CreateSightingResponse {
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
