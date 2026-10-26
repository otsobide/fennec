//! Response for the create-mac-address use case.

use crate::mac_address::application::find_mac_address::find_mac_address_response::MacAddressErrorEntry;

/// Response envelope returned by [`CreateMacAddressCommandHandler`].
///
/// [`CreateMacAddressCommandHandler`]: super::create_mac_address_command_handler::CreateMacAddressCommandHandler
pub struct CreateMacAddressResponse {
    pub error: Option<MacAddressErrorEntry>,
}
