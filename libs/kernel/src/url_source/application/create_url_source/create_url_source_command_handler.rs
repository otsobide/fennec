//! [`CommandHandler`] for the create-url-source use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::url_source::application::find_url_source::find_url_source_response::UrlSourceErrorEntry;
use crate::url_source::domain::errors::url_source_repository_error::UrlSourceRepositoryError;

use super::create_url_source_command::CreateUrlSourceCommand;
use super::create_url_source_response::CreateUrlSourceResponse;
use super::url_source_creator::UrlSourceCreator;

/// [`CommandHandler`] that processes [`CreateUrlSourceCommand`]s by delegating
/// to [`UrlSourceCreator`].
pub struct CreateUrlSourceCommandHandler {
    creator: UrlSourceCreator,
}

impl CreateUrlSourceCommandHandler {
    pub fn new(creator: UrlSourceCreator) -> Self {
        Self { creator }
    }
}

#[async_trait]
impl CommandHandler<CreateUrlSourceCommand> for CreateUrlSourceCommandHandler {
    type Response = CreateUrlSourceResponse;

    async fn handle(
        &self,
        command: CreateUrlSourceCommand,
    ) -> Result<Self::Response, CommandBusError> {
        match self
            .creator
            .execute(command.id, command.url, command.format, command.polling_interval)
            .await
        {
            Ok(()) => Ok(CreateUrlSourceResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    UrlSourceRepositoryError::NotFound => "NotFound",
                    UrlSourceRepositoryError::AlreadyExists => "AlreadyExists",
                    UrlSourceRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(CreateUrlSourceResponse {
                    error: Some(UrlSourceErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
