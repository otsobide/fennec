//! [`CommandHandler`] for the delete-ipv6-network use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::ipv6_network::application::find_ipv6_network::find_ipv6_network_response::Ipv6NetworkErrorEntry;
use crate::ipv6_network::domain::errors::ipv6_network_repository_error::Ipv6NetworkRepositoryError;

use super::delete_ipv6_network_command::DeleteIpv6NetworkCommand;
use super::delete_ipv6_network_response::DeleteIpv6NetworkResponse;
use super::ipv6_network_deleter::Ipv6NetworkDeleter;

/// [`CommandHandler`] that processes [`DeleteIpv6NetworkCommand`]s by
/// delegating to [`Ipv6NetworkDeleter`].
pub struct DeleteIpv6NetworkCommandHandler {
    deleter: Ipv6NetworkDeleter,
}

impl DeleteIpv6NetworkCommandHandler {
    pub fn new(deleter: Ipv6NetworkDeleter) -> Self {
        Self { deleter }
    }
}

#[async_trait]
impl CommandHandler<DeleteIpv6NetworkCommand> for DeleteIpv6NetworkCommandHandler {
    type Response = DeleteIpv6NetworkResponse;

    async fn handle(
        &self,
        command: DeleteIpv6NetworkCommand,
    ) -> Result<Self::Response, CommandBusError> {
        match self.deleter.execute(command.id).await {
            Ok(()) => Ok(DeleteIpv6NetworkResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    Ipv6NetworkRepositoryError::NotFound => "NotFound",
                    Ipv6NetworkRepositoryError::AlreadyExists => "AlreadyExists",
                    Ipv6NetworkRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(DeleteIpv6NetworkResponse {
                    error: Some(Ipv6NetworkErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
