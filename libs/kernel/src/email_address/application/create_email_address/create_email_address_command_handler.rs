//! [`CommandHandler`] for the create-email-address use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::email_address::application::find_email_address::find_email_address_response::EmailAddressErrorEntry;
use crate::email_address::domain::errors::email_address_repository_error::EmailAddressRepositoryError;

use super::create_email_address_command::CreateEmailAddressCommand;
use super::create_email_address_response::CreateEmailAddressResponse;
use super::email_address_creator::EmailAddressCreator;

/// [`CommandHandler`] that processes [`CreateEmailAddressCommand`]s by
/// delegating to [`EmailAddressCreator`].
pub struct CreateEmailAddressCommandHandler {
    creator: EmailAddressCreator,
}

impl CreateEmailAddressCommandHandler {
    pub fn new(creator: EmailAddressCreator) -> Self {
        Self { creator }
    }
}

#[async_trait]
impl CommandHandler<CreateEmailAddressCommand> for CreateEmailAddressCommandHandler {
    type Response = CreateEmailAddressResponse;

    async fn handle(
        &self,
        command: CreateEmailAddressCommand,
    ) -> Result<Self::Response, CommandBusError> {
        match self.creator.execute(command.value).await {
            Ok(()) => Ok(CreateEmailAddressResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    EmailAddressRepositoryError::NotFound => "NotFound",
                    EmailAddressRepositoryError::AlreadyExists => "AlreadyExists",
                    EmailAddressRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(CreateEmailAddressResponse {
                    error: Some(EmailAddressErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
