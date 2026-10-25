//! [`CommandHandler`] for the delete-ipv4-address use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::ipv4_address::application::find_ipv4_address::find_ipv4_address_response::Ipv4AddressErrorEntry;
use crate::ipv4_address::domain::errors::ipv4_address_repository_error::Ipv4AddressRepositoryError;

use super::delete_ipv4_address_command::DeleteIpv4AddressCommand;
use super::delete_ipv4_address_response::DeleteIpv4AddressResponse;
use super::ipv4_address_deleter::Ipv4AddressDeleter;

/// [`CommandHandler`] that processes [`DeleteIpv4AddressCommand`]s by
/// delegating to [`Ipv4AddressDeleter`].
pub struct DeleteIpv4AddressCommandHandler {
    deleter: Ipv4AddressDeleter,
}

impl DeleteIpv4AddressCommandHandler {
    pub fn new(deleter: Ipv4AddressDeleter) -> Self {
        Self { deleter }
    }
}

#[async_trait]
impl CommandHandler<DeleteIpv4AddressCommand> for DeleteIpv4AddressCommandHandler {
    type Response = DeleteIpv4AddressResponse;

    async fn handle(
        &self,
        command: DeleteIpv4AddressCommand,
    ) -> Result<Self::Response, CommandBusError> {
        match self.deleter.execute(command.id).await {
            Ok(()) => Ok(DeleteIpv4AddressResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    Ipv4AddressRepositoryError::NotFound => "NotFound",
                    Ipv4AddressRepositoryError::AlreadyExists => "AlreadyExists",
                    Ipv4AddressRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(DeleteIpv4AddressResponse {
                    error: Some(Ipv4AddressErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
