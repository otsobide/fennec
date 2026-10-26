//! In-memory implementation of [`Ipv6NetworkRepository`].

use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use uuid::Uuid;

use crate::ipv6_network::domain::entities::ipv6_network::Ipv6Network;
use crate::ipv6_network::domain::errors::ipv6_network_repository_error::Ipv6NetworkRepositoryError;
use crate::ipv6_network::domain::repositories::ipv6_network_repository::Ipv6NetworkRepository;
use crate::ipv6_network::domain::value_objects::ipv6_network_id::Ipv6NetworkId;

/// In-memory implementation of [`Ipv6NetworkRepository`] backed by a
/// [`HashMap`] protected by a [`Mutex`].
///
/// Keyed by the derived id, so registering the same network twice fails with
/// [`Ipv6NetworkRepositoryError::AlreadyExists`], exactly as the unique key
/// of a real database would. Intended for tests and local development.
pub struct InMemoryIpv6NetworkRepository {
    store: Mutex<HashMap<Uuid, Ipv6Network>>,
}

impl InMemoryIpv6NetworkRepository {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryIpv6NetworkRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Ipv6NetworkRepository for InMemoryIpv6NetworkRepository {
    async fn save(&self, ipv6_network: &Ipv6Network) -> Result<(), Ipv6NetworkRepositoryError> {
        let mut store = self.store.lock().unwrap();
        let id = *ipv6_network.id().value();
        if store.contains_key(&id) {
            return Err(Ipv6NetworkRepositoryError::AlreadyExists);
        }
        store.insert(id, ipv6_network.clone());
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &Ipv6NetworkId,
    ) -> Result<Option<Ipv6Network>, Ipv6NetworkRepositoryError> {
        let store = self.store.lock().unwrap();
        Ok(store.get(id.value()).cloned())
    }

    async fn delete(&self, id: &Ipv6NetworkId) -> Result<(), Ipv6NetworkRepositoryError> {
        let mut store = self.store.lock().unwrap();
        if store.remove(id.value()).is_none() {
            return Err(Ipv6NetworkRepositoryError::NotFound);
        }
        Ok(())
    }
}
