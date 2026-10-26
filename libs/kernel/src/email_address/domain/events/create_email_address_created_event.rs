//! Factory function for [`EmailAddressCreatedEvent`].

use crate::email_address::domain::entities::email_address::EmailAddress;
use crate::email_address::domain::errors::email_address_repository_error::EmailAddressRepositoryError;
use crate::email_address::domain::events::email_address_created_event::EmailAddressCreatedEvent;

/// Creates an [`EmailAddressCreatedEvent`] from the given email address.
pub fn create_email_address_created_event(
    email_address: &EmailAddress,
) -> Result<EmailAddressCreatedEvent, EmailAddressRepositoryError> {
    Ok(EmailAddressCreatedEvent::new(email_address))
}
