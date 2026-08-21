//! [`CommandHandler`] for the update-url-source use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::url_source::application::find_url_source::find_url_source_response::UrlSourceErrorEntry;
use crate::url_source::domain::errors::url_source_repository_error::UrlSourceRepositoryError;

use super::update_url_source_command::UpdateUrlSourceCommand;
use super::update_url_source_response::UpdateUrlSourceResponse;
use super::url_source_updater::UrlSourceUpdater;

/// [`CommandHandler`] that processes [`UpdateUrlSourceCommand`]s by delegating
/// to [`UrlSourceUpdater`].
pub struct UpdateUrlSourceCommandHandler {
    updater: UrlSourceUpdater,
}

impl UpdateUrlSourceCommandHandler {
    pub fn new(updater: UrlSourceUpdater) -> Self {
        Self { updater }
    }
}

#[async_trait]
impl CommandHandler<UpdateUrlSourceCommand> for UpdateUrlSourceCommandHandler {
    type Response = UpdateUrlSourceResponse;

    async fn handle(
        &self,
        command: UpdateUrlSourceCommand,
    ) -> Result<Self::Response, CommandBusError> {
        match self
            .updater
            .execute(
                command.id,
                command.url,
                command.format,
                command.polling_interval,
            )
            .await
        {
            Ok(()) => Ok(UpdateUrlSourceResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    UrlSourceRepositoryError::NotFound => "NotFound",
                    UrlSourceRepositoryError::AlreadyExists => "AlreadyExists",
                    UrlSourceRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(UpdateUrlSourceResponse {
                    error: Some(UrlSourceErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
