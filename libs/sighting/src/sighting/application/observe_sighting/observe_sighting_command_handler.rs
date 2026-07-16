//! [`CommandHandler`] for the observe-sighting use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::sighting::application::find_sighting::find_sighting_response::SightingErrorEntry;
use crate::sighting::domain::errors::sighting_repository_error::SightingRepositoryError;

use super::observe_sighting_command::ObserveSightingCommand;
use super::observe_sighting_response::ObserveSightingResponse;
use super::sighting_observer::SightingObserver;

/// [`CommandHandler`] that processes [`ObserveSightingCommand`]s by delegating
/// to [`SightingObserver`].
pub struct ObserveSightingCommandHandler {
    observer: SightingObserver,
}

impl ObserveSightingCommandHandler {
    pub fn new(observer: SightingObserver) -> Self {
        Self { observer }
    }
}

#[async_trait]
impl CommandHandler<ObserveSightingCommand> for ObserveSightingCommandHandler {
    type Response = ObserveSightingResponse;

    async fn handle(
        &self,
        command: ObserveSightingCommand,
    ) -> Result<Self::Response, CommandBusError> {
        match self.observer.execute(command.id, command.observed_at).await {
            Ok(()) => Ok(ObserveSightingResponse { error: None }),
            Err(e) => {
                let (concept, existing_id) = match &e {
                    SightingRepositoryError::NotFound => ("NotFound", None),
                    SightingRepositoryError::IdAlreadyExists => ("AlreadyExists", None),
                    SightingRepositoryError::PairAlreadyExists { existing_id } => {
                        ("PairAlreadyExists", Some(existing_id.to_string()))
                    }
                    SightingRepositoryError::Unexpected(_) => ("Unexpected", None),
                };
                Ok(ObserveSightingResponse {
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
