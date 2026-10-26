//! [`CommandHandler`] for the delete-ipv4-network use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::ipv4_network::application::find_ipv4_network::find_ipv4_network_response::Ipv4NetworkErrorEntry;
use crate::ipv4_network::domain::errors::ipv4_network_repository_error::Ipv4NetworkRepositoryError;

use super::delete_ipv4_network_command::DeleteIpv4NetworkCommand;
use super::delete_ipv4_network_response::DeleteIpv4NetworkResponse;
use super::ipv4_network_deleter::Ipv4NetworkDeleter;

/// [`CommandHandler`] that processes [`DeleteIpv4NetworkCommand`]s by
/// delegating to [`Ipv4NetworkDeleter`].
pub struct DeleteIpv4NetworkCommandHandler {
    deleter: Ipv4NetworkDeleter,
}

impl DeleteIpv4NetworkCommandHandler {
    pub fn new(deleter: Ipv4NetworkDeleter) -> Self {
        Self { deleter }
    }
}

#[async_trait]
impl CommandHandler<DeleteIpv4NetworkCommand> for DeleteIpv4NetworkCommandHandler {
    type Response = DeleteIpv4NetworkResponse;

    async fn handle(
        &self,
        command: DeleteIpv4NetworkCommand,
    ) -> Result<Self::Response, CommandBusError> {
        match self.deleter.execute(command.id).await {
            Ok(()) => Ok(DeleteIpv4NetworkResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    Ipv4NetworkRepositoryError::NotFound => "NotFound",
                    Ipv4NetworkRepositoryError::AlreadyExists => "AlreadyExists",
                    Ipv4NetworkRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(DeleteIpv4NetworkResponse {
                    error: Some(Ipv4NetworkErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
