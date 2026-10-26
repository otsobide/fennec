//! [`CommandHandler`] for the create-ipv6-network use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::ipv6_network::application::find_ipv6_network::find_ipv6_network_response::Ipv6NetworkErrorEntry;
use crate::ipv6_network::domain::errors::ipv6_network_repository_error::Ipv6NetworkRepositoryError;

use super::create_ipv6_network_command::CreateIpv6NetworkCommand;
use super::create_ipv6_network_response::CreateIpv6NetworkResponse;
use super::ipv6_network_creator::Ipv6NetworkCreator;

/// [`CommandHandler`] that processes [`CreateIpv6NetworkCommand`]s by
/// delegating to [`Ipv6NetworkCreator`].
pub struct CreateIpv6NetworkCommandHandler {
    creator: Ipv6NetworkCreator,
}

impl CreateIpv6NetworkCommandHandler {
    pub fn new(creator: Ipv6NetworkCreator) -> Self {
        Self { creator }
    }
}

#[async_trait]
impl CommandHandler<CreateIpv6NetworkCommand> for CreateIpv6NetworkCommandHandler {
    type Response = CreateIpv6NetworkResponse;

    async fn handle(
        &self,
        command: CreateIpv6NetworkCommand,
    ) -> Result<Self::Response, CommandBusError> {
        match self.creator.execute(command.value).await {
            Ok(()) => Ok(CreateIpv6NetworkResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    Ipv6NetworkRepositoryError::NotFound => "NotFound",
                    Ipv6NetworkRepositoryError::AlreadyExists => "AlreadyExists",
                    Ipv6NetworkRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(CreateIpv6NetworkResponse {
                    error: Some(Ipv6NetworkErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
