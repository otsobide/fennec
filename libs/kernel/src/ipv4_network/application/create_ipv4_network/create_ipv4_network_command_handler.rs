//! [`CommandHandler`] for the create-ipv4-network use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::ipv4_network::application::find_ipv4_network::find_ipv4_network_response::Ipv4NetworkErrorEntry;
use crate::ipv4_network::domain::errors::ipv4_network_repository_error::Ipv4NetworkRepositoryError;

use super::create_ipv4_network_command::CreateIpv4NetworkCommand;
use super::create_ipv4_network_response::CreateIpv4NetworkResponse;
use super::ipv4_network_creator::Ipv4NetworkCreator;

/// [`CommandHandler`] that processes [`CreateIpv4NetworkCommand`]s by
/// delegating to [`Ipv4NetworkCreator`].
pub struct CreateIpv4NetworkCommandHandler {
    creator: Ipv4NetworkCreator,
}

impl CreateIpv4NetworkCommandHandler {
    pub fn new(creator: Ipv4NetworkCreator) -> Self {
        Self { creator }
    }
}

#[async_trait]
impl CommandHandler<CreateIpv4NetworkCommand> for CreateIpv4NetworkCommandHandler {
    type Response = CreateIpv4NetworkResponse;

    async fn handle(
        &self,
        command: CreateIpv4NetworkCommand,
    ) -> Result<Self::Response, CommandBusError> {
        match self.creator.execute(command.value).await {
            Ok(()) => Ok(CreateIpv4NetworkResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    Ipv4NetworkRepositoryError::NotFound => "NotFound",
                    Ipv4NetworkRepositoryError::AlreadyExists => "AlreadyExists",
                    Ipv4NetworkRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(CreateIpv4NetworkResponse {
                    error: Some(Ipv4NetworkErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
