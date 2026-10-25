use std::sync::Mutex;

use async_trait::async_trait;
use uuid::Uuid;

use kernel::sighting::domain::entities::sighting::Sighting;
use kernel::sighting::domain::errors::sighting_repository_error::SightingRepositoryError;
use kernel::sighting::domain::repositories::sighting_repository::SightingRepository;
use kernel::sighting::domain::value_objects::sighting_id::SightingId;
use kernel::sighting::domain::value_objects::sighting_ioc_id::SightingIocId;
use kernel::sighting::domain::value_objects::sighting_source_id::SightingSourceId;

#[allow(dead_code)]
pub enum SaveBehavior {
    Succeeds,
    FailsWithIdAlreadyExists,
    FailsWithPairAlreadyExists { existing_id: SightingId },
    FailsWithUnexpected(String),
}

#[allow(dead_code)]
pub enum FindByIdBehavior {
    ReturnsNone,
    ReturnsSighting(Mutex<Option<Sighting>>),
    FailsWithUnexpected(String),
}

#[allow(dead_code)]
pub enum UpdateBehavior {
    Succeeds,
    FailsWithNotFound,
    FailsWithUnexpected(String),
}

#[allow(dead_code)]
pub enum FindByPairBehavior {
    ReturnsNone,
    ReturnsSighting(Mutex<Option<Sighting>>),
    FailsWithUnexpected(String),
}

#[allow(dead_code)]
pub enum FindByIocBehavior {
    ReturnsEmpty,
    ReturnsSightings(Mutex<Option<Vec<Sighting>>>),
    FailsWithUnexpected(String),
}

#[allow(dead_code)]
pub enum FindBySourceBehavior {
    ReturnsEmpty,
    ReturnsSightings(Mutex<Option<Vec<Sighting>>>),
    FailsWithUnexpected(String),
}

#[allow(dead_code)]
pub enum DeleteBehavior {
    Succeeds,
    FailsWithNotFound,
    FailsWithUnexpected(String),
}

pub struct SightingRepositoryMock {
    save_behavior: SaveBehavior,
    find_by_id_behavior: FindByIdBehavior,
    update_behavior: UpdateBehavior,
    find_by_pair_behavior: FindByPairBehavior,
    find_by_ioc_behavior: FindByIocBehavior,
    find_by_source_behavior: FindBySourceBehavior,
    delete_behavior: DeleteBehavior,
    saved_ids: Mutex<Vec<Uuid>>,
    update_call_count: Mutex<u32>,
    delete_call_count: Mutex<u32>,
}

#[allow(dead_code)]
impl SightingRepositoryMock {
    fn base() -> Self {
        Self {
            save_behavior: SaveBehavior::Succeeds,
            find_by_id_behavior: FindByIdBehavior::ReturnsNone,
            update_behavior: UpdateBehavior::Succeeds,
            find_by_pair_behavior: FindByPairBehavior::ReturnsNone,
            find_by_ioc_behavior: FindByIocBehavior::ReturnsEmpty,
            find_by_source_behavior: FindBySourceBehavior::ReturnsEmpty,
            delete_behavior: DeleteBehavior::Succeeds,
            saved_ids: Mutex::new(vec![]),
            update_call_count: Mutex::new(0),
            delete_call_count: Mutex::new(0),
        }
    }

    pub fn that_succeeds() -> Self {
        Self::base()
    }

    pub fn that_fails_with_id_already_exists() -> Self {
        Self {
            save_behavior: SaveBehavior::FailsWithIdAlreadyExists,
            ..Self::base()
        }
    }

    pub fn that_fails_with_pair_already_exists(existing_id: SightingId) -> Self {
        Self {
            save_behavior: SaveBehavior::FailsWithPairAlreadyExists { existing_id },
            ..Self::base()
        }
    }

    pub fn that_fails_on_save(message: String) -> Self {
        Self {
            save_behavior: SaveBehavior::FailsWithUnexpected(message),
            ..Self::base()
        }
    }

    pub fn that_returns_sighting(sighting: Sighting) -> Self {
        Self {
            find_by_id_behavior: FindByIdBehavior::ReturnsSighting(Mutex::new(Some(sighting))),
            ..Self::base()
        }
    }

    pub fn that_returns_sighting_but_update_fails(sighting: Sighting) -> Self {
        Self {
            find_by_id_behavior: FindByIdBehavior::ReturnsSighting(Mutex::new(Some(sighting))),
            update_behavior: UpdateBehavior::FailsWithNotFound,
            ..Self::base()
        }
    }

    pub fn that_returns_sighting_but_delete_fails(sighting: Sighting) -> Self {
        Self {
            find_by_id_behavior: FindByIdBehavior::ReturnsSighting(Mutex::new(Some(sighting))),
            delete_behavior: DeleteBehavior::FailsWithNotFound,
            ..Self::base()
        }
    }

    pub fn that_finds_nothing() -> Self {
        Self::base()
    }

    pub fn that_fails_on_find(message: String) -> Self {
        Self {
            find_by_id_behavior: FindByIdBehavior::FailsWithUnexpected(message),
            ..Self::base()
        }
    }

    pub fn that_returns_pair(sighting: Option<Sighting>) -> Self {
        Self {
            find_by_pair_behavior: FindByPairBehavior::ReturnsSighting(Mutex::new(sighting)),
            ..Self::base()
        }
    }

    pub fn that_returns_sightings_for_ioc(sightings: Vec<Sighting>) -> Self {
        Self {
            find_by_ioc_behavior: FindByIocBehavior::ReturnsSightings(Mutex::new(Some(sightings))),
            ..Self::base()
        }
    }

    pub fn that_fails_on_find_by_ioc(message: String) -> Self {
        Self {
            find_by_ioc_behavior: FindByIocBehavior::FailsWithUnexpected(message),
            ..Self::base()
        }
    }

    pub fn that_returns_sightings_for_source(sightings: Vec<Sighting>) -> Self {
        Self {
            find_by_source_behavior: FindBySourceBehavior::ReturnsSightings(Mutex::new(Some(
                sightings,
            ))),
            ..Self::base()
        }
    }

    pub fn that_fails_on_find_by_source(message: String) -> Self {
        Self {
            find_by_source_behavior: FindBySourceBehavior::FailsWithUnexpected(message),
            ..Self::base()
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
impl SightingRepository for SightingRepositoryMock {
    async fn save(&self, sighting: &Sighting) -> Result<(), SightingRepositoryError> {
        match &self.save_behavior {
            SaveBehavior::FailsWithIdAlreadyExists => Err(SightingRepositoryError::IdAlreadyExists),
            SaveBehavior::FailsWithPairAlreadyExists { existing_id } => {
                Err(SightingRepositoryError::PairAlreadyExists {
                    existing_id: existing_id.clone(),
                })
            }
            SaveBehavior::FailsWithUnexpected(msg) => {
                Err(SightingRepositoryError::Unexpected(msg.clone()))
            }
            SaveBehavior::Succeeds => {
                self.saved_ids.lock().unwrap().push(*sighting.id().value());
                Ok(())
            }
        }
    }

    async fn update(&self, _sighting: &Sighting) -> Result<(), SightingRepositoryError> {
        *self.update_call_count.lock().unwrap() += 1;
        match &self.update_behavior {
            UpdateBehavior::Succeeds => Ok(()),
            UpdateBehavior::FailsWithNotFound => Err(SightingRepositoryError::NotFound),
            UpdateBehavior::FailsWithUnexpected(msg) => {
                Err(SightingRepositoryError::Unexpected(msg.clone()))
            }
        }
    }

    async fn find_by_id(
        &self,
        _id: &SightingId,
    ) -> Result<Option<Sighting>, SightingRepositoryError> {
        match &self.find_by_id_behavior {
            FindByIdBehavior::ReturnsNone => Ok(None),
            FindByIdBehavior::ReturnsSighting(cell) => Ok(cell.lock().unwrap().take()),
            FindByIdBehavior::FailsWithUnexpected(msg) => {
                Err(SightingRepositoryError::Unexpected(msg.clone()))
            }
        }
    }

    async fn find_by_ioc_and_source(
        &self,
        _ioc_id: &SightingIocId,
        _source_id: &SightingSourceId,
    ) -> Result<Option<Sighting>, SightingRepositoryError> {
        match &self.find_by_pair_behavior {
            FindByPairBehavior::ReturnsNone => Ok(None),
            FindByPairBehavior::ReturnsSighting(cell) => Ok(cell.lock().unwrap().take()),
            FindByPairBehavior::FailsWithUnexpected(msg) => {
                Err(SightingRepositoryError::Unexpected(msg.clone()))
            }
        }
    }

    async fn find_by_ioc_id(
        &self,
        _ioc_id: &SightingIocId,
    ) -> Result<Vec<Sighting>, SightingRepositoryError> {
        match &self.find_by_ioc_behavior {
            FindByIocBehavior::ReturnsEmpty => Ok(Vec::new()),
            FindByIocBehavior::ReturnsSightings(cell) => {
                Ok(cell.lock().unwrap().take().unwrap_or_default())
            }
            FindByIocBehavior::FailsWithUnexpected(msg) => {
                Err(SightingRepositoryError::Unexpected(msg.clone()))
            }
        }
    }

    async fn find_by_source_id(
        &self,
        _source_id: &SightingSourceId,
    ) -> Result<Vec<Sighting>, SightingRepositoryError> {
        match &self.find_by_source_behavior {
            FindBySourceBehavior::ReturnsEmpty => Ok(Vec::new()),
            FindBySourceBehavior::ReturnsSightings(cell) => {
                Ok(cell.lock().unwrap().take().unwrap_or_default())
            }
            FindBySourceBehavior::FailsWithUnexpected(msg) => {
                Err(SightingRepositoryError::Unexpected(msg.clone()))
            }
        }
    }

    async fn delete(&self, _id: &SightingId) -> Result<(), SightingRepositoryError> {
        *self.delete_call_count.lock().unwrap() += 1;
        match &self.delete_behavior {
            DeleteBehavior::Succeeds => Ok(()),
            DeleteBehavior::FailsWithNotFound => Err(SightingRepositoryError::NotFound),
            DeleteBehavior::FailsWithUnexpected(msg) => {
                Err(SightingRepositoryError::Unexpected(msg.clone()))
            }
        }
    }
}
