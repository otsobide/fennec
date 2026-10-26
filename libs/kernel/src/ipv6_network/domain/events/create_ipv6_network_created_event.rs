//! Factory function for [`Ipv6NetworkCreatedEvent`].

use crate::ipv6_network::domain::entities::ipv6_network::Ipv6Network;
use crate::ipv6_network::domain::errors::ipv6_network_repository_error::Ipv6NetworkRepositoryError;
use crate::ipv6_network::domain::events::ipv6_network_created_event::Ipv6NetworkCreatedEvent;

/// Creates an [`Ipv6NetworkCreatedEvent`] from the given IPv6 network.
pub fn create_ipv6_network_created_event(
    ipv6_network: &Ipv6Network,
) -> Result<Ipv6NetworkCreatedEvent, Ipv6NetworkRepositoryError> {
    Ok(Ipv6NetworkCreatedEvent::new(ipv6_network))
}
