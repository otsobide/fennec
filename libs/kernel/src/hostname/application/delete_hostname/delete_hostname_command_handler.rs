//! [`CommandHandler`] for the delete-hostname use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::hostname::application::find_hostname::find_hostname_response::HostnameErrorEntry;
use crate::hostname::domain::errors::hostname_repository_error::HostnameRepositoryError;

use super::delete_hostname_command::DeleteHostnameCommand;
use super::delete_hostname_response::DeleteHostnameResponse;
use super::hostname_deleter::HostnameDeleter;

/// [`CommandHandler`] that processes [`DeleteHostnameCommand`]s by
/// delegating to [`HostnameDeleter`].
pub struct DeleteHostnameCommandHandler {
    deleter: HostnameDeleter,
}

impl DeleteHostnameCommandHandler {
    pub fn new(deleter: HostnameDeleter) -> Self {
        Self { deleter }
    }
}

#[async_trait]
impl CommandHandler<DeleteHostnameCommand> for DeleteHostnameCommandHandler {
    type Response = DeleteHostnameResponse;

    async fn handle(
        &self,
        command: DeleteHostnameCommand,
    ) -> Result<Self::Response, CommandBusError> {
        match self.deleter.execute(command.id).await {
            Ok(()) => Ok(DeleteHostnameResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    HostnameRepositoryError::NotFound => "NotFound",
                    HostnameRepositoryError::AlreadyExists => "AlreadyExists",
                    HostnameRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(DeleteHostnameResponse {
                    error: Some(HostnameErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
