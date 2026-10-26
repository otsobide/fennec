//! [`CommandHandler`] for the create-hostname use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::hostname::application::find_hostname::find_hostname_response::HostnameErrorEntry;
use crate::hostname::domain::errors::hostname_repository_error::HostnameRepositoryError;

use super::create_hostname_command::CreateHostnameCommand;
use super::create_hostname_response::CreateHostnameResponse;
use super::hostname_creator::HostnameCreator;

/// [`CommandHandler`] that processes [`CreateHostnameCommand`]s by
/// delegating to [`HostnameCreator`].
pub struct CreateHostnameCommandHandler {
    creator: HostnameCreator,
}

impl CreateHostnameCommandHandler {
    pub fn new(creator: HostnameCreator) -> Self {
        Self { creator }
    }
}

#[async_trait]
impl CommandHandler<CreateHostnameCommand> for CreateHostnameCommandHandler {
    type Response = CreateHostnameResponse;

    async fn handle(
        &self,
        command: CreateHostnameCommand,
    ) -> Result<Self::Response, CommandBusError> {
        match self.creator.execute(command.value).await {
            Ok(()) => Ok(CreateHostnameResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    HostnameRepositoryError::NotFound => "NotFound",
                    HostnameRepositoryError::AlreadyExists => "AlreadyExists",
                    HostnameRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(CreateHostnameResponse {
                    error: Some(HostnameErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
