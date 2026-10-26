//! Response for the create-ipv6-address use case.

use crate::ipv6_address::application::find_ipv6_address::find_ipv6_address_response::Ipv6AddressErrorEntry;

/// Response envelope returned by [`CreateIpv6AddressCommandHandler`].
///
/// [`CreateIpv6AddressCommandHandler`]: super::create_ipv6_address_command_handler::CreateIpv6AddressCommandHandler
pub struct CreateIpv6AddressResponse {
    pub error: Option<Ipv6AddressErrorEntry>,
}
