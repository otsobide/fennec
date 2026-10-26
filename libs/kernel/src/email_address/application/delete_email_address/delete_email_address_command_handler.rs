//! [`CommandHandler`] for the delete-email-address use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::email_address::application::find_email_address::find_email_address_response::EmailAddressErrorEntry;
use crate::email_address::domain::errors::email_address_repository_error::EmailAddressRepositoryError;

use super::delete_email_address_command::DeleteEmailAddressCommand;
use super::delete_email_address_response::DeleteEmailAddressResponse;
use super::email_address_deleter::EmailAddressDeleter;

/// [`CommandHandler`] that processes [`DeleteEmailAddressCommand`]s by
/// delegating to [`EmailAddressDeleter`].
pub struct DeleteEmailAddressCommandHandler {
    deleter: EmailAddressDeleter,
}

impl DeleteEmailAddressCommandHandler {
    pub fn new(deleter: EmailAddressDeleter) -> Self {
        Self { deleter }
    }
}

#[async_trait]
impl CommandHandler<DeleteEmailAddressCommand> for DeleteEmailAddressCommandHandler {
    type Response = DeleteEmailAddressResponse;

    async fn handle(
        &self,
        command: DeleteEmailAddressCommand,
    ) -> Result<Self::Response, CommandBusError> {
        match self.deleter.execute(command.id).await {
            Ok(()) => Ok(DeleteEmailAddressResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    EmailAddressRepositoryError::NotFound => "NotFound",
                    EmailAddressRepositoryError::AlreadyExists => "AlreadyExists",
                    EmailAddressRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(DeleteEmailAddressResponse {
                    error: Some(EmailAddressErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
