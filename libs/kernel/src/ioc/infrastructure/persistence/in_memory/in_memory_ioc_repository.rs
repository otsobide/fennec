//! In-memory implementation of [`IocRepository`].

use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use uuid::Uuid;

use crate::ioc::domain::entities::ioc::Ioc;
use crate::ioc::domain::errors::ioc_repository_error::IocRepositoryError;
use crate::ioc::domain::repositories::ioc_repository::IocRepository;
use crate::ioc::domain::value_objects::ioc_id::IocId;

/// In-memory implementation of [`IocRepository`] backed by a [`HashMap`]
/// protected by a [`Mutex`].
///
/// Intended for tests and local development.
pub struct InMemoryIocRepository {
    store: Mutex<HashMap<Uuid, Ioc>>,
}

impl InMemoryIocRepository {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryIocRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl IocRepository for InMemoryIocRepository {
    async fn save(&self, ioc: &Ioc) -> Result<(), IocRepositoryError> {
        let mut store = self.store.lock().unwrap();
        let id = *ioc.id().value();
        if store.contains_key(&id) {
            return Err(IocRepositoryError::AlreadyExists);
        }
        store.insert(id, ioc.clone());
        Ok(())
    }

    async fn find_by_id(&self, id: &IocId) -> Result<Option<Ioc>, IocRepositoryError> {
        let store = self.store.lock().unwrap();
        Ok(store.get(id.value()).cloned())
    }

    async fn delete(&self, id: &IocId) -> Result<(), IocRepositoryError> {
        let mut store = self.store.lock().unwrap();
        if store.remove(id.value()).is_none() {
            return Err(IocRepositoryError::NotFound);
        }
        Ok(())
    }
}
