//! Response for the delete-email-address use case.

use crate::email_address::application::find_email_address::find_email_address_response::EmailAddressErrorEntry;

/// Response envelope returned by [`DeleteEmailAddressCommandHandler`].
///
/// [`DeleteEmailAddressCommandHandler`]: super::delete_email_address_command_handler::DeleteEmailAddressCommandHandler
pub struct DeleteEmailAddressResponse {
    pub error: Option<EmailAddressErrorEntry>,
}
