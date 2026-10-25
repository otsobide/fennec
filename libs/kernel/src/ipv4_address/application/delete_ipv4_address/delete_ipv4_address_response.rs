//! Response for the delete-ipv4-address use case.

use crate::ipv4_address::application::find_ipv4_address::find_ipv4_address_response::Ipv4AddressErrorEntry;

/// Response envelope returned by [`DeleteIpv4AddressCommandHandler`].
///
/// [`DeleteIpv4AddressCommandHandler`]: super::delete_ipv4_address_command_handler::DeleteIpv4AddressCommandHandler
pub struct DeleteIpv4AddressResponse {
    pub error: Option<Ipv4AddressErrorEntry>,
}
