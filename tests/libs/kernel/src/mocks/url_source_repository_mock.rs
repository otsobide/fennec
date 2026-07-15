use std::sync::Mutex;

use async_trait::async_trait;
use uuid::Uuid;

use kernel::url_source::domain::entities::url_source::UrlSource;
use kernel::url_source::domain::errors::url_source_repository_error::UrlSourceRepositoryError;
use kernel::url_source::domain::repositories::url_source_repository::UrlSourceRepository;
use kernel::url_source::domain::value_objects::url_source_id::UrlSourceId;

pub enum SaveBehavior {
    Succeeds,
    FailsWithAlreadyExists,
}

#[allow(dead_code)]
pub enum FindByIdBehavior {
    ReturnsNone,
    ReturnsUrlSource(Mutex<Option<UrlSource>>),
    FailsWithUnexpected(String),
}

#[allow(dead_code)]
pub enum UpdateBehavior {
    Succeeds,
    FailsWithNotFound,
    FailsWithUnexpected(String),
}

pub struct UrlSourceRepositoryMock {
    save_behavior: SaveBehavior,
    find_by_id_behavior: FindByIdBehavior,
    update_behavior: UpdateBehavior,
    saved_ids: Mutex<Vec<Uuid>>,
    update_call_count: Mutex<u32>,
    delete_call_count: Mutex<u32>,
}

#[allow(dead_code)]
impl UrlSourceRepositoryMock {
    pub fn that_succeeds() -> Self {
        Self {
            save_behavior: SaveBehavior::Succeeds,
            find_by_id_behavior: FindByIdBehavior::ReturnsNone,
            update_behavior: UpdateBehavior::Succeeds,
            saved_ids: Mutex::new(vec![]),
            update_call_count: Mutex::new(0),
            delete_call_count: Mutex::new(0),
        }
    }

    pub fn that_fails_with_already_exists() -> Self {
        Self {
            save_behavior: SaveBehavior::FailsWithAlreadyExists,
            find_by_id_behavior: FindByIdBehavior::ReturnsNone,
            update_behavior: UpdateBehavior::Succeeds,
            saved_ids: Mutex::new(vec![]),
            update_call_count: Mutex::new(0),
            delete_call_count: Mutex::new(0),
        }
    }

    pub fn that_returns_url_source(url_source: UrlSource) -> Self {
        Self {
            save_behavior: SaveBehavior::Succeeds,
            find_by_id_behavior: FindByIdBehavior::ReturnsUrlSource(Mutex::new(Some(url_source))),
            update_behavior: UpdateBehavior::Succeeds,
            saved_ids: Mutex::new(vec![]),
            update_call_count: Mutex::new(0),
            delete_call_count: Mutex::new(0),
        }
    }

    pub fn that_returns_url_source_but_update_fails(url_source: UrlSource) -> Self {
        Self {
            save_behavior: SaveBehavior::Succeeds,
            find_by_id_behavior: FindByIdBehavior::ReturnsUrlSource(Mutex::new(Some(url_source))),
            update_behavior: UpdateBehavior::FailsWithNotFound,
            saved_ids: Mutex::new(vec![]),
            update_call_count: Mutex::new(0),
            delete_call_count: Mutex::new(0),
        }
    }

    pub fn that_finds_nothing() -> Self {
        Self {
            save_behavior: SaveBehavior::Succeeds,
            find_by_id_behavior: FindByIdBehavior::ReturnsNone,
            update_behavior: UpdateBehavior::Succeeds,
            saved_ids: Mutex::new(vec![]),
            update_call_count: Mutex::new(0),
            delete_call_count: Mutex::new(0),
        }
    }

    pub fn that_fails_on_find(message: String) -> Self {
        Self {
            save_behavior: SaveBehavior::Succeeds,
            find_by_id_behavior: FindByIdBehavior::FailsWithUnexpected(message),
            update_behavior: UpdateBehavior::Succeeds,
            saved_ids: Mutex::new(vec![]),
            update_call_count: Mutex::new(0),
            delete_call_count: Mutex::new(0),
        }
    }

    pub fn saved_ids(&self) -> Vec<Uuid> {
        self.saved_ids.lock().unwrap().clone()
    }

    pub fn update_call_count(&self) -> u32 {
        *self.update_call_count.lock().unwrap()
    }

    pub fn delete_call_count(&self) -> u32 {
        *self.delete_call_count.lock().unwrap()
    }
}

#[async_trait]
impl UrlSourceRepository for UrlSourceRepositoryMock {
    async fn save(&self, url_source: &UrlSource) -> Result<(), UrlSourceRepositoryError> {
        match &self.save_behavior {
            SaveBehavior::FailsWithAlreadyExists => Err(UrlSourceRepositoryError::AlreadyExists),
            SaveBehavior::Succeeds => {
                self.saved_ids.lock().unwrap().push(*url_source.id().value());
                Ok(())
            }
        }
    }

    async fn find_by_id(
        &self,
        _id: &UrlSourceId,
    ) -> Result<Option<UrlSource>, UrlSourceRepositoryError> {
        match &self.find_by_id_behavior {
            FindByIdBehavior::ReturnsNone => Ok(None),
            FindByIdBehavior::ReturnsUrlSource(cell) => Ok(cell.lock().unwrap().take()),
            FindByIdBehavior::FailsWithUnexpected(msg) => {
                Err(UrlSourceRepositoryError::Unexpected(msg.clone()))
            }
        }
    }

    async fn update(&self, _url_source: &UrlSource) -> Result<(), UrlSourceRepositoryError> {
        *self.update_call_count.lock().unwrap() += 1;
        match &self.update_behavior {
            UpdateBehavior::Succeeds => Ok(()),
            UpdateBehavior::FailsWithNotFound => Err(UrlSourceRepositoryError::NotFound),
            UpdateBehavior::FailsWithUnexpected(msg) => {
                Err(UrlSourceRepositoryError::Unexpected(msg.clone()))
            }
        }
    }

    async fn delete(&self, _id: &UrlSourceId) -> Result<(), UrlSourceRepositoryError> {
        *self.delete_call_count.lock().unwrap() += 1;
        Ok(())
    }
}
