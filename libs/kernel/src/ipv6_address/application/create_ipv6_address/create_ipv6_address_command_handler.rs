//! [`CommandHandler`] for the create-ipv6-address use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::ipv6_address::application::find_ipv6_address::find_ipv6_address_response::Ipv6AddressErrorEntry;
use crate::ipv6_address::domain::errors::ipv6_address_repository_error::Ipv6AddressRepositoryError;

use super::create_ipv6_address_command::CreateIpv6AddressCommand;
use super::create_ipv6_address_response::CreateIpv6AddressResponse;
use super::ipv6_address_creator::Ipv6AddressCreator;

/// [`CommandHandler`] that processes [`CreateIpv6AddressCommand`]s by
/// delegating to [`Ipv6AddressCreator`].
pub struct CreateIpv6AddressCommandHandler {
    creator: Ipv6AddressCreator,
}

impl CreateIpv6AddressCommandHandler {
    pub fn new(creator: Ipv6AddressCreator) -> Self {
        Self { creator }
    }
}

#[async_trait]
impl CommandHandler<CreateIpv6AddressCommand> for CreateIpv6AddressCommandHandler {
    type Response = CreateIpv6AddressResponse;

    async fn handle(
        &self,
        command: CreateIpv6AddressCommand,
    ) -> Result<Self::Response, CommandBusError> {
        match self.creator.execute(command.value).await {
            Ok(()) => Ok(CreateIpv6AddressResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    Ipv6AddressRepositoryError::NotFound => "NotFound",
                    Ipv6AddressRepositoryError::AlreadyExists => "AlreadyExists",
                    Ipv6AddressRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(CreateIpv6AddressResponse {
                    error: Some(Ipv6AddressErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
