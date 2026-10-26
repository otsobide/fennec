//! Command for deleting an email address.

use shared_cqrs::command::domain::command::Command;

use crate::email_address::domain::value_objects::email_address_id::EmailAddressId;

/// Command that requests the deletion of the email address with the given id.
pub struct DeleteEmailAddressCommand {
    pub id: EmailAddressId,
}

impl Command for DeleteEmailAddressCommand {}
