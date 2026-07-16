use std::sync::Mutex;

use async_trait::async_trait;
use uuid::Uuid;

use ioc::ioc::domain::entities::ioc::Ioc;
use ioc::ioc::domain::errors::ioc_repository_error::IocRepositoryError;
use ioc::ioc::domain::repositories::ioc_repository::IocRepository;
use ioc::ioc::domain::value_objects::ioc_id::IocId;

pub enum SaveBehavior {
    Succeeds,
    FailsWithAlreadyExists,
}

#[allow(dead_code)]
pub enum FindByIdBehavior {
    ReturnsNone,
    ReturnsIoc(Mutex<Option<Ioc>>),
    FailsWithUnexpected(String),
}

pub struct IocRepositoryMock {
    save_behavior: SaveBehavior,
    find_by_id_behavior: FindByIdBehavior,
    saved_ids: Mutex<Vec<Uuid>>,
    delete_call_count: Mutex<u32>,
}

#[allow(dead_code)]
impl IocRepositoryMock {
    pub fn that_succeeds() -> Self {
        Self {
            save_behavior: SaveBehavior::Succeeds,
            find_by_id_behavior: FindByIdBehavior::ReturnsNone,
            saved_ids: Mutex::new(vec![]),
            delete_call_count: Mutex::new(0),
        }
    }

    pub fn that_fails_with_already_exists() -> Self {
        Self {
            save_behavior: SaveBehavior::FailsWithAlreadyExists,
            find_by_id_behavior: FindByIdBehavior::ReturnsNone,
            saved_ids: Mutex::new(vec![]),
            delete_call_count: Mutex::new(0),
        }
    }

    pub fn that_returns_ioc(ioc: Ioc) -> Self {
        Self {
            save_behavior: SaveBehavior::Succeeds,
            find_by_id_behavior: FindByIdBehavior::ReturnsIoc(Mutex::new(Some(ioc))),
            saved_ids: Mutex::new(vec![]),
            delete_call_count: Mutex::new(0),
        }
    }

    pub fn that_finds_nothing() -> Self {
        Self {
            save_behavior: SaveBehavior::Succeeds,
            find_by_id_behavior: FindByIdBehavior::ReturnsNone,
            saved_ids: Mutex::new(vec![]),
            delete_call_count: Mutex::new(0),
        }
    }

    pub fn that_fails_on_find(message: String) -> Self {
        Self {
            save_behavior: SaveBehavior::Succeeds,
            find_by_id_behavior: FindByIdBehavior::FailsWithUnexpected(message),
            saved_ids: Mutex::new(vec![]),
            delete_call_count: Mutex::new(0),
        }
    }

    pub fn saved_ids(&self) -> Vec<Uuid> {
        self.saved_ids.lock().unwrap().clone()
    }

    pub fn delete_call_count(&self) -> u32 {
        *self.delete_call_count.lock().unwrap()
    }
}

#[async_trait]
impl IocRepository for IocRepositoryMock {
    async fn save(&self, ioc: &Ioc) -> Result<(), IocRepositoryError> {
        match &self.save_behavior {
            SaveBehavior::FailsWithAlreadyExists => Err(IocRepositoryError::AlreadyExists),
            SaveBehavior::Succeeds => {
                self.saved_ids.lock().unwrap().push(*ioc.id().value());
                Ok(())
            }
        }
    }

    async fn find_by_id(&self, _id: &IocId) -> Result<Option<Ioc>, IocRepositoryError> {
        match &self.find_by_id_behavior {
            FindByIdBehavior::ReturnsNone => Ok(None),
            FindByIdBehavior::ReturnsIoc(cell) => Ok(cell.lock().unwrap().take()),
            FindByIdBehavior::FailsWithUnexpected(msg) => {
                Err(IocRepositoryError::Unexpected(msg.clone()))
            }
        }
    }

    async fn delete(&self, _id: &IocId) -> Result<(), IocRepositoryError> {
        *self.delete_call_count.lock().unwrap() += 1;
        Ok(())
    }
}
