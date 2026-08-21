//! [`CommandHandler`] for the create-ioc use case.

use async_trait::async_trait;
use shared_cqrs::command::domain::command_bus_error::CommandBusError;
use shared_cqrs::command::domain::command_handler::CommandHandler;

use crate::ioc::application::find_ioc::find_ioc_response::IocErrorEntry;
use crate::ioc::domain::errors::ioc_repository_error::IocRepositoryError;

use super::create_ioc_command::CreateIocCommand;
use super::create_ioc_response::CreateIocResponse;
use super::ioc_creator::IocCreator;

/// [`CommandHandler`] that processes [`CreateIocCommand`]s by delegating to
/// [`IocCreator`].
pub struct CreateIocCommandHandler {
    creator: IocCreator,
}

impl CreateIocCommandHandler {
    pub fn new(creator: IocCreator) -> Self {
        Self { creator }
    }
}

#[async_trait]
impl CommandHandler<CreateIocCommand> for CreateIocCommandHandler {
    type Response = CreateIocResponse;

    async fn handle(&self, command: CreateIocCommand) -> Result<Self::Response, CommandBusError> {
        match self
            .creator
            .execute(command.id, command.ioc_type, command.value)
            .await
        {
            Ok(()) => Ok(CreateIocResponse { error: None }),
            Err(e) => {
                let concept = match &e {
                    IocRepositoryError::NotFound => "NotFound",
                    IocRepositoryError::AlreadyExists => "AlreadyExists",
                    IocRepositoryError::Unexpected(_) => "Unexpected",
                };
                Ok(CreateIocResponse {
                    error: Some(IocErrorEntry {
                        message: e.to_string(),
                        concept: concept.to_string(),
                    }),
                })
            }
        }
    }
}
