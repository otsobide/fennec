//! Factory function for [`Ipv6NetworkDeletedEvent`].

use crate::ipv6_network::domain::entities::ipv6_network::Ipv6Network;
use crate::ipv6_network::domain::errors::ipv6_network_repository_error::Ipv6NetworkRepositoryError;
use crate::ipv6_network::domain::events::ipv6_network_deleted_event::Ipv6NetworkDeletedEvent;

/// Creates an [`Ipv6NetworkDeletedEvent`] from the deleted IPv6 network.
pub fn create_ipv6_network_deleted_event(
    ipv6_network: &Ipv6Network,
) -> Result<Ipv6NetworkDeletedEvent, Ipv6NetworkRepositoryError> {
    Ok(Ipv6NetworkDeletedEvent::new(ipv6_network))
}
