//! Command for deleting a url source.

use shared_cqrs::command::domain::command::Command;

use crate::url_source::domain::value_objects::url_source_id::UrlSourceId;

/// Command that requests the deletion of a url source by id.
pub struct DeleteUrlSourceCommand {
    pub id: UrlSourceId,
}

impl Command for DeleteUrlSourceCommand {}
