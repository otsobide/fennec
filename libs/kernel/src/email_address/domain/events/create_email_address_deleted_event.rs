//! Factory function for [`EmailAddressDeletedEvent`].

use crate::email_address::domain::entities::email_address::EmailAddress;
use crate::email_address::domain::errors::email_address_repository_error::EmailAddressRepositoryError;
use crate::email_address::domain::events::email_address_deleted_event::EmailAddressDeletedEvent;

/// Creates an [`EmailAddressDeletedEvent`] from the deleted email address.
pub fn create_email_address_deleted_event(
    email_address: &EmailAddress,
) -> Result<EmailAddressDeletedEvent, EmailAddressRepositoryError> {
    Ok(EmailAddressDeletedEvent::new(email_address))
}
