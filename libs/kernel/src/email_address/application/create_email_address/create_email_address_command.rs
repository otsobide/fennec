//! Command for registering an email address.

use shared_cqrs::command::domain::command::Command;

use crate::email_address::domain::value_objects::email_address_value::EmailAddressValue;

/// Command that requests the registration of an email address.
///
/// It carries no id: the id of an email address is derived from its value by
/// [`EmailAddressCreator`].
///
/// [`EmailAddressCreator`]: super::email_address_creator::EmailAddressCreator
pub struct CreateEmailAddressCommand {
    pub value: EmailAddressValue,
}

impl Command for CreateEmailAddressCommand {}
