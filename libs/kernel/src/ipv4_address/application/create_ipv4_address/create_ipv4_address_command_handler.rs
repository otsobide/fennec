//! [`CommandHandler`] for the create-ipv4-address use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::ipv4_address::application::find_ipv4_address::find_ipv4_address_response::Ipv4AddressErrorEntry;
use crate::ipv4_address::domain::errors::ipv4_address_repository_error::Ipv4AddressRepositoryError;

use super::create_ipv4_address_command::CreateIpv4AddressCommand;
use super::create_ipv4_address_response::CreateIpv4AddressResponse;
use super::ipv4_address_creator::Ipv4AddressCreator;

/// [`CommandHandler`] that processes [`CreateIpv4AddressCommand`]s by
/// delegating to [`Ipv4AddressCreator`].
pub struct CreateIpv4AddressCommandHandler {
    creator: Ipv4AddressCreator,
}

impl CreateIpv4AddressCommandHandler {
    pub fn new(creator: Ipv4AddressCreator) -> Self {
        Self { creator }
    }
}

#[async_trait]
impl CommandHandler<CreateIpv4AddressCommand> for CreateIpv4AddressCommandHandler {
    type Response = CreateIpv4AddressResponse;

    async fn handle(
        &self,
        command: CreateIpv4AddressCommand,
    ) -> Result<Self::Response, CommandBusError> {
        match self.creator.execute(command.value).await {
            Ok(()) => Ok(CreateIpv4AddressResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    Ipv4AddressRepositoryError::NotFound => "NotFound",
                    Ipv4AddressRepositoryError::AlreadyExists => "AlreadyExists",
                    Ipv4AddressRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(CreateIpv4AddressResponse {
                    error: Some(Ipv4AddressErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
