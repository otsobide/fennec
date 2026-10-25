//! Response for the create-ipv4-address use case.

use crate::ipv4_address::application::find_ipv4_address::find_ipv4_address_response::Ipv4AddressErrorEntry;

/// Response envelope returned by [`CreateIpv4AddressCommandHandler`].
///
/// [`CreateIpv4AddressCommandHandler`]: super::create_ipv4_address_command_handler::CreateIpv4AddressCommandHandler
pub struct CreateIpv4AddressResponse {
    pub error: Option<Ipv4AddressErrorEntry>,
}
