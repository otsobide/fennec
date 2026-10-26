//! In-memory implementation of [`DomainNameRepository`].

use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain_name::domain::entities::domain_name::DomainName;
use crate::domain_name::domain::errors::domain_name_repository_error::DomainNameRepositoryError;
use crate::domain_name::domain::repositories::domain_name_repository::DomainNameRepository;
use crate::domain_name::domain::value_objects::domain_name_id::DomainNameId;

/// In-memory implementation of [`DomainNameRepository`] backed by a
/// [`HashMap`] protected by a [`Mutex`].
///
/// Keyed by the derived id, so registering the same name twice fails with
/// [`DomainNameRepositoryError::AlreadyExists`], exactly as the unique key
/// of a real database would. Intended for tests and local development.
pub struct InMemoryDomainNameRepository {
    store: Mutex<HashMap<Uuid, DomainName>>,
}

impl InMemoryDomainNameRepository {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryDomainNameRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DomainNameRepository for InMemoryDomainNameRepository {
    async fn save(&self, domain_name: &DomainName) -> Result<(), DomainNameRepositoryError> {
        let mut store = self.store.lock().unwrap();
        let id = *domain_name.id().value();
        if store.contains_key(&id) {
            return Err(DomainNameRepositoryError::AlreadyExists);
        }
        store.insert(id, domain_name.clone());
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &DomainNameId,
    ) -> Result<Option<DomainName>, DomainNameRepositoryError> {
        let store = self.store.lock().unwrap();
        Ok(store.get(id.value()).cloned())
    }

    async fn delete(&self, id: &DomainNameId) -> Result<(), DomainNameRepositoryError> {
        let mut store = self.store.lock().unwrap();
        if store.remove(id.value()).is_none() {
            return Err(DomainNameRepositoryError::NotFound);
        }
        Ok(())
    }
}
