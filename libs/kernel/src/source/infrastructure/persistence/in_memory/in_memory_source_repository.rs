//! In-memory implementation of [`SourceRepository`].

use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use uuid::Uuid;

use crate::source::domain::entities::source::Source;
use crate::source::domain::errors::source_repository_error::SourceRepositoryError;
use crate::source::domain::repositories::source_repository::SourceRepository;
use crate::source::domain::value_objects::source_id::SourceId;

/// An in-memory implementation of [`SourceRepository`] backed by a
/// [`HashMap`] protected by a [`Mutex`].
///
/// Intended for use in tests and local development.
pub struct InMemorySourceRepository {
    store: Mutex<HashMap<Uuid, Source>>,
}

impl InMemorySourceRepository {
    /// Creates a new empty `InMemorySourceRepository`.
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for InMemorySourceRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SourceRepository for InMemorySourceRepository {
    async fn save(&self, source: &Source) -> Result<(), SourceRepositoryError> {
        let mut store = self.store.lock().unwrap();
        let id = *source.id().value();
        if store.contains_key(&id) {
            return Err(SourceRepositoryError::AlreadyExists);
        }
        store.insert(id, source.clone());
        Ok(())
    }

    async fn find_by_id(&self, id: &SourceId) -> Result<Option<Source>, SourceRepositoryError> {
        let store = self.store.lock().unwrap();
        Ok(store.get(id.value()).cloned())
    }

    async fn update(&self, source: &Source) -> Result<(), SourceRepositoryError> {
        let mut store = self.store.lock().unwrap();
        let id = *source.id().value();
        if !store.contains_key(&id) {
            return Err(SourceRepositoryError::NotFound);
        }
        store.insert(id, source.clone());
        Ok(())
    }

    async fn delete(&self, id: &SourceId) -> Result<(), SourceRepositoryError> {
        let mut store = self.store.lock().unwrap();
        if store.remove(id.value()).is_none() {
            return Err(SourceRepositoryError::NotFound);
        }
        Ok(())
    }
}
