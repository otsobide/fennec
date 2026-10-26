//! Response for the create-email-address use case.

use crate::email_address::application::find_email_address::find_email_address_response::EmailAddressErrorEntry;

/// Response envelope returned by [`CreateEmailAddressCommandHandler`].
///
/// [`CreateEmailAddressCommandHandler`]: super::create_email_address_command_handler::CreateEmailAddressCommandHandler
pub struct CreateEmailAddressResponse {
    pub error: Option<EmailAddressErrorEntry>,
}
