//! Response for the delete-ipv6-address use case.

use crate::ipv6_address::application::find_ipv6_address::find_ipv6_address_response::Ipv6AddressErrorEntry;

/// Response envelope returned by [`DeleteIpv6AddressCommandHandler`].
///
/// [`DeleteIpv6AddressCommandHandler`]: super::delete_ipv6_address_command_handler::DeleteIpv6AddressCommandHandler
pub struct DeleteIpv6AddressResponse {
    pub error: Option<Ipv6AddressErrorEntry>,
}
