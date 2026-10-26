//! [`CommandHandler`] for the delete-ipv6-address use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::ipv6_address::application::find_ipv6_address::find_ipv6_address_response::Ipv6AddressErrorEntry;
use crate::ipv6_address::domain::errors::ipv6_address_repository_error::Ipv6AddressRepositoryError;

use super::delete_ipv6_address_command::DeleteIpv6AddressCommand;
use super::delete_ipv6_address_response::DeleteIpv6AddressResponse;
use super::ipv6_address_deleter::Ipv6AddressDeleter;

/// [`CommandHandler`] that processes [`DeleteIpv6AddressCommand`]s by
/// delegating to [`Ipv6AddressDeleter`].
pub struct DeleteIpv6AddressCommandHandler {
    deleter: Ipv6AddressDeleter,
}

impl DeleteIpv6AddressCommandHandler {
    pub fn new(deleter: Ipv6AddressDeleter) -> Self {
        Self { deleter }
    }
}

#[async_trait]
impl CommandHandler<DeleteIpv6AddressCommand> for DeleteIpv6AddressCommandHandler {
    type Response = DeleteIpv6AddressResponse;

    async fn handle(
        &self,
        command: DeleteIpv6AddressCommand,
    ) -> Result<Self::Response, CommandBusError> {
        match self.deleter.execute(command.id).await {
            Ok(()) => Ok(DeleteIpv6AddressResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    Ipv6AddressRepositoryError::NotFound => "NotFound",
                    Ipv6AddressRepositoryError::AlreadyExists => "AlreadyExists",
                    Ipv6AddressRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(DeleteIpv6AddressResponse {
                    error: Some(Ipv6AddressErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
