//! Response for the delete-mac-address use case.

use crate::mac_address::application::find_mac_address::find_mac_address_response::MacAddressErrorEntry;

/// Response envelope returned by [`DeleteMacAddressCommandHandler`].
///
/// [`DeleteMacAddressCommandHandler`]: super::delete_mac_address_command_handler::DeleteMacAddressCommandHandler
pub struct DeleteMacAddressResponse {
    pub error: Option<MacAddressErrorEntry>,
}
