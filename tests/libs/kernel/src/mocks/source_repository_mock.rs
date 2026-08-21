use std::sync::Mutex;

use async_trait::async_trait;
use uuid::Uuid;

use kernel::source::domain::entities::source::Source;
use kernel::source::domain::errors::source_repository_error::SourceRepositoryError;
use kernel::source::domain::repositories::source_repository::SourceRepository;
use kernel::source::domain::value_objects::source_id::SourceId;

pub enum SaveBehavior {
    Succeeds,
    FailsWithAlreadyExists,
}

#[allow(dead_code)]
pub enum FindByIdBehavior {
    ReturnsNone,
    ReturnsSource(Mutex<Option<Source>>),
    FailsWithUnexpected(String),
}

#[allow(dead_code)]
pub enum UpdateBehavior {
    Succeeds,
    FailsWithNotFound,
    FailsWithUnexpected(String),
}

pub struct SourceRepositoryMock {
    save_behavior: SaveBehavior,
    find_by_id_behavior: FindByIdBehavior,
    update_behavior: UpdateBehavior,
    saved_ids: Mutex<Vec<Uuid>>,
    update_call_count: Mutex<u32>,
    delete_call_count: Mutex<u32>,
}

#[allow(dead_code)]
impl SourceRepositoryMock {
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

    pub fn that_returns_source(source: Source) -> Self {
        Self {
            save_behavior: SaveBehavior::Succeeds,
            find_by_id_behavior: FindByIdBehavior::ReturnsSource(Mutex::new(Some(source))),
            update_behavior: UpdateBehavior::Succeeds,
            saved_ids: Mutex::new(vec![]),
            update_call_count: Mutex::new(0),
            delete_call_count: Mutex::new(0),
        }
    }

    pub fn that_returns_source_but_update_fails(source: Source) -> Self {
        Self {
            save_behavior: SaveBehavior::Succeeds,
            find_by_id_behavior: FindByIdBehavior::ReturnsSource(Mutex::new(Some(source))),
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
impl SourceRepository for SourceRepositoryMock {
    async fn save(&self, source: &Source) -> Result<(), SourceRepositoryError> {
        match &self.save_behavior {
            SaveBehavior::FailsWithAlreadyExists => Err(SourceRepositoryError::AlreadyExists),
            SaveBehavior::Succeeds => {
                self.saved_ids.lock().unwrap().push(*source.id().value());
                Ok(())
            }
        }
    }

    async fn find_by_id(&self, _id: &SourceId) -> Result<Option<Source>, SourceRepositoryError> {
        match &self.find_by_id_behavior {
            FindByIdBehavior::ReturnsNone => Ok(None),
            FindByIdBehavior::ReturnsSource(cell) => Ok(cell.lock().unwrap().take()),
            FindByIdBehavior::FailsWithUnexpected(msg) => {
                Err(SourceRepositoryError::Unexpected(msg.clone()))
            }
        }
    }

    async fn update(&self, _source: &Source) -> Result<(), SourceRepositoryError> {
        *self.update_call_count.lock().unwrap() += 1;
        match &self.update_behavior {
            UpdateBehavior::Succeeds => Ok(()),
            UpdateBehavior::FailsWithNotFound => Err(SourceRepositoryError::NotFound),
            UpdateBehavior::FailsWithUnexpected(msg) => {
                Err(SourceRepositoryError::Unexpected(msg.clone()))
            }
        }
    }

    async fn delete(&self, _id: &SourceId) -> Result<(), SourceRepositoryError> {
        *self.delete_call_count.lock().unwrap() += 1;
        Ok(())
    }
}
