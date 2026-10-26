//! [`CommandHandler`] for the create-mac-address use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::mac_address::application::find_mac_address::find_mac_address_response::MacAddressErrorEntry;
use crate::mac_address::domain::errors::mac_address_repository_error::MacAddressRepositoryError;

use super::create_mac_address_command::CreateMacAddressCommand;
use super::create_mac_address_response::CreateMacAddressResponse;
use super::mac_address_creator::MacAddressCreator;

/// [`CommandHandler`] that processes [`CreateMacAddressCommand`]s by
/// delegating to [`MacAddressCreator`].
pub struct CreateMacAddressCommandHandler {
    creator: MacAddressCreator,
}

impl CreateMacAddressCommandHandler {
    pub fn new(creator: MacAddressCreator) -> Self {
        Self { creator }
    }
}

#[async_trait]
impl CommandHandler<CreateMacAddressCommand> for CreateMacAddressCommandHandler {
    type Response = CreateMacAddressResponse;

    async fn handle(
        &self,
        command: CreateMacAddressCommand,
    ) -> Result<Self::Response, CommandBusError> {
        match self.creator.execute(command.value).await {
            Ok(()) => Ok(CreateMacAddressResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    MacAddressRepositoryError::NotFound => "NotFound",
                    MacAddressRepositoryError::AlreadyExists => "AlreadyExists",
                    MacAddressRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(CreateMacAddressResponse {
                    error: Some(MacAddressErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
