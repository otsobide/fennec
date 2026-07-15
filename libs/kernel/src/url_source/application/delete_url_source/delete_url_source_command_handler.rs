//! [`CommandHandler`] for the delete-url-source use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::url_source::application::find_url_source::find_url_source_response::UrlSourceErrorEntry;
use crate::url_source::domain::errors::url_source_repository_error::UrlSourceRepositoryError;

use super::delete_url_source_command::DeleteUrlSourceCommand;
use super::delete_url_source_response::DeleteUrlSourceResponse;
use super::url_source_deleter::UrlSourceDeleter;

/// [`CommandHandler`] that processes [`DeleteUrlSourceCommand`]s by delegating
/// to [`UrlSourceDeleter`].
pub struct DeleteUrlSourceCommandHandler {
    deleter: UrlSourceDeleter,
}

impl DeleteUrlSourceCommandHandler {
    pub fn new(deleter: UrlSourceDeleter) -> Self {
        Self { deleter }
    }
}

#[async_trait]
impl CommandHandler<DeleteUrlSourceCommand> for DeleteUrlSourceCommandHandler {
    type Response = DeleteUrlSourceResponse;

    async fn handle(
        &self,
        command: DeleteUrlSourceCommand,
    ) -> Result<Self::Response, CommandBusError> {
        match self.deleter.execute(command.id).await {
            Ok(()) => Ok(DeleteUrlSourceResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    UrlSourceRepositoryError::NotFound => "NotFound",
                    UrlSourceRepositoryError::AlreadyExists => "AlreadyExists",
                    UrlSourceRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(DeleteUrlSourceResponse {
                    error: Some(UrlSourceErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
