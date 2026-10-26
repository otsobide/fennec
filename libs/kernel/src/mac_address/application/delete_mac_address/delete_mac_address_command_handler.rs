//! [`CommandHandler`] for the delete-mac-address use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::mac_address::application::find_mac_address::find_mac_address_response::MacAddressErrorEntry;
use crate::mac_address::domain::errors::mac_address_repository_error::MacAddressRepositoryError;

use super::delete_mac_address_command::DeleteMacAddressCommand;
use super::delete_mac_address_response::DeleteMacAddressResponse;
use super::mac_address_deleter::MacAddressDeleter;

/// [`CommandHandler`] that processes [`DeleteMacAddressCommand`]s by
/// delegating to [`MacAddressDeleter`].
pub struct DeleteMacAddressCommandHandler {
    deleter: MacAddressDeleter,
}

impl DeleteMacAddressCommandHandler {
    pub fn new(deleter: MacAddressDeleter) -> Self {
        Self { deleter }
    }
}

#[async_trait]
impl CommandHandler<DeleteMacAddressCommand> for DeleteMacAddressCommandHandler {
    type Response = DeleteMacAddressResponse;

    async fn handle(
        &self,
        command: DeleteMacAddressCommand,
    ) -> Result<Self::Response, CommandBusError> {
        match self.deleter.execute(command.id).await {
            Ok(()) => Ok(DeleteMacAddressResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    MacAddressRepositoryError::NotFound => "NotFound",
                    MacAddressRepositoryError::AlreadyExists => "AlreadyExists",
                    MacAddressRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(DeleteMacAddressResponse {
                    error: Some(MacAddressErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
