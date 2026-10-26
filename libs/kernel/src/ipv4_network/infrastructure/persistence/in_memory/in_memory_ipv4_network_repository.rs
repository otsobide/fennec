//! In-memory implementation of [`Ipv4NetworkRepository`].

use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use uuid::Uuid;

use crate::ipv4_network::domain::entities::ipv4_network::Ipv4Network;
use crate::ipv4_network::domain::errors::ipv4_network_repository_error::Ipv4NetworkRepositoryError;
use crate::ipv4_network::domain::repositories::ipv4_network_repository::Ipv4NetworkRepository;
use crate::ipv4_network::domain::value_objects::ipv4_network_id::Ipv4NetworkId;

/// In-memory implementation of [`Ipv4NetworkRepository`] backed by a
/// [`HashMap`] protected by a [`Mutex`].
///
/// Keyed by the derived id, so registering the same network twice fails with
/// [`Ipv4NetworkRepositoryError::AlreadyExists`], exactly as the unique key
/// of a real database would. Intended for tests and local development.
pub struct InMemoryIpv4NetworkRepository {
    store: Mutex<HashMap<Uuid, Ipv4Network>>,
}

impl InMemoryIpv4NetworkRepository {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryIpv4NetworkRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Ipv4NetworkRepository for InMemoryIpv4NetworkRepository {
    async fn save(&self, ipv4_network: &Ipv4Network) -> Result<(), Ipv4NetworkRepositoryError> {
        let mut store = self.store.lock().unwrap();
        let id = *ipv4_network.id().value();
        if store.contains_key(&id) {
            return Err(Ipv4NetworkRepositoryError::AlreadyExists);
        }
        store.insert(id, ipv4_network.clone());
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &Ipv4NetworkId,
    ) -> Result<Option<Ipv4Network>, Ipv4NetworkRepositoryError> {
        let store = self.store.lock().unwrap();
        Ok(store.get(id.value()).cloned())
    }

    async fn delete(&self, id: &Ipv4NetworkId) -> Result<(), Ipv4NetworkRepositoryError> {
        let mut store = self.store.lock().unwrap();
        if store.remove(id.value()).is_none() {
            return Err(Ipv4NetworkRepositoryError::NotFound);
        }
        Ok(())
    }
}
