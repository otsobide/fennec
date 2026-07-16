//! [`CommandHandler`] for the delete-ioc use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::ioc::application::find_ioc::find_ioc_response::IocErrorEntry;
use crate::ioc::domain::errors::ioc_repository_error::IocRepositoryError;

use super::delete_ioc_command::DeleteIocCommand;
use super::delete_ioc_response::DeleteIocResponse;
use super::ioc_deleter::IocDeleter;

/// [`CommandHandler`] that processes [`DeleteIocCommand`]s by delegating to
/// [`IocDeleter`].
pub struct DeleteIocCommandHandler {
    deleter: IocDeleter,
}

impl DeleteIocCommandHandler {
    pub fn new(deleter: IocDeleter) -> Self {
        Self { deleter }
    }
}

#[async_trait]
impl CommandHandler<DeleteIocCommand> for DeleteIocCommandHandler {
    type Response = DeleteIocResponse;

    async fn handle(
        &self,
        command: DeleteIocCommand,
    ) -> Result<Self::Response, CommandBusError> {
        match self.deleter.execute(command.id).await {
            Ok(()) => Ok(DeleteIocResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    IocRepositoryError::NotFound => "NotFound",
                    IocRepositoryError::AlreadyExists => "AlreadyExists",
                    IocRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(DeleteIocResponse {
                    error: Some(IocErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
