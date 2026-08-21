//! Command for deleting an ioc.

use shared_cqrs::command::domain::command::Command;

use crate::ioc::domain::value_objects::ioc_id::IocId;

/// Command that requests the deletion of an ioc by id.
pub struct DeleteIocCommand {
    pub id: IocId,
}

impl Command for DeleteIocCommand {}
