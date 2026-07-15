//! In-memory implementation of [`UrlSourceRepository`].

use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use uuid::Uuid;

use crate::url_source::domain::entities::url_source::UrlSource;
use crate::url_source::domain::errors::url_source_repository_error::UrlSourceRepositoryError;
use crate::url_source::domain::repositories::url_source_repository::UrlSourceRepository;
use crate::url_source::domain::value_objects::url_source_id::UrlSourceId;

/// In-memory implementation of [`UrlSourceRepository`] backed by a
/// [`HashMap`] protected by a [`Mutex`].
///
/// Intended for tests and local development.
pub struct InMemoryUrlSourceRepository {
    store: Mutex<HashMap<Uuid, UrlSource>>,
}

impl InMemoryUrlSourceRepository {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryUrlSourceRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UrlSourceRepository for InMemoryUrlSourceRepository {
    async fn save(&self, url_source: &UrlSource) -> Result<(), UrlSourceRepositoryError> {
        let mut store = self.store.lock().unwrap();
        let id = *url_source.id().value();
        if store.contains_key(&id) {
            return Err(UrlSourceRepositoryError::AlreadyExists);
        }
        store.insert(id, url_source.clone());
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &UrlSourceId,
    ) -> Result<Option<UrlSource>, UrlSourceRepositoryError> {
        let store = self.store.lock().unwrap();
        Ok(store.get(id.value()).cloned())
    }

    async fn update(&self, url_source: &UrlSource) -> Result<(), UrlSourceRepositoryError> {
        let mut store = self.store.lock().unwrap();
        let id = *url_source.id().value();
        if !store.contains_key(&id) {
            return Err(UrlSourceRepositoryError::NotFound);
        }
        store.insert(id, url_source.clone());
        Ok(())
    }

    async fn delete(&self, id: &UrlSourceId) -> Result<(), UrlSourceRepositoryError> {
        let mut store = self.store.lock().unwrap();
        if store.remove(id.value()).is_none() {
            return Err(UrlSourceRepositoryError::NotFound);
        }
        Ok(())
    }
}
