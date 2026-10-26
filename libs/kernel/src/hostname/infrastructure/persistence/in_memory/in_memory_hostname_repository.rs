//! In-memory implementation of [`HostnameRepository`].

use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use uuid::Uuid;

use crate::hostname::domain::entities::hostname::Hostname;
use crate::hostname::domain::errors::hostname_repository_error::HostnameRepositoryError;
use crate::hostname::domain::repositories::hostname_repository::HostnameRepository;
use crate::hostname::domain::value_objects::hostname_id::HostnameId;

/// In-memory implementation of [`HostnameRepository`] backed by a
/// [`HashMap`] protected by a [`Mutex`].
///
/// Keyed by the derived id, so registering the same hostname twice fails with
/// [`HostnameRepositoryError::AlreadyExists`], exactly as the unique key
/// of a real database would. Intended for tests and local development.
pub struct InMemoryHostnameRepository {
    store: Mutex<HashMap<Uuid, Hostname>>,
}

impl InMemoryHostnameRepository {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryHostnameRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl HostnameRepository for InMemoryHostnameRepository {
    async fn save(&self, hostname: &Hostname) -> Result<(), HostnameRepositoryError> {
        let mut store = self.store.lock().unwrap();
        let id = *hostname.id().value();
        if store.contains_key(&id) {
            return Err(HostnameRepositoryError::AlreadyExists);
        }
        store.insert(id, hostname.clone());
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &HostnameId,
    ) -> Result<Option<Hostname>, HostnameRepositoryError> {
        let store = self.store.lock().unwrap();
        Ok(store.get(id.value()).cloned())
    }

    async fn delete(&self, id: &HostnameId) -> Result<(), HostnameRepositoryError> {
        let mut store = self.store.lock().unwrap();
        if store.remove(id.value()).is_none() {
            return Err(HostnameRepositoryError::NotFound);
        }
        Ok(())
    }
}
