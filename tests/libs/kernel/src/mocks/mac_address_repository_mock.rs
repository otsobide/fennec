use std::sync::Mutex;

use async_trait::async_trait;
use uuid::Uuid;

use kernel::mac_address::domain::entities::mac_address::MacAddress;
use kernel::mac_address::domain::errors::mac_address_repository_error::MacAddressRepositoryError;
use kernel::mac_address::domain::repositories::mac_address_repository::MacAddressRepository;
use kernel::mac_address::domain::value_objects::mac_address_id::MacAddressId;

pub enum SaveBehavior {
    Succeeds,
    FailsWithAlreadyExists,
    FailsWithUnexpected(String),
}

pub enum DeleteBehavior {
    Succeeds,
    FailsWithUnexpected(String),
}

#[allow(dead_code)]
pub enum FindByIdBehavior {
    ReturnsNone,
    ReturnsMacAddress(Mutex<Option<MacAddress>>),
    FailsWithUnexpected(String),
}

pub struct MacAddressRepositoryMock {
    save_behavior: SaveBehavior,
    find_by_id_behavior: FindByIdBehavior,
    delete_behavior: DeleteBehavior,
    saved_ids: Mutex<Vec<Uuid>>,
    delete_call_count: Mutex<u32>,
}

#[allow(dead_code)]
impl MacAddressRepositoryMock {
    pub fn that_succeeds() -> Self {
        Self::with(SaveBehavior::Succeeds, FindByIdBehavior::ReturnsNone)
    }

    pub fn that_fails_with_already_exists() -> Self {
        Self::with(
            SaveBehavior::FailsWithAlreadyExists,
            FindByIdBehavior::ReturnsNone,
        )
    }

    pub fn that_returns_mac_address(mac_address: MacAddress) -> Self {
        Self::with(
            SaveBehavior::Succeeds,
            FindByIdBehavior::ReturnsMacAddress(Mutex::new(Some(mac_address))),
        )
    }

    pub fn that_fails_on_save(message: String) -> Self {
        Self::with(
            SaveBehavior::FailsWithUnexpected(message),
            FindByIdBehavior::ReturnsNone,
        )
    }

    pub fn that_returns_mac_address_but_delete_fails(
        mac_address: MacAddress,
        message: String,
    ) -> Self {
        Self {
            delete_behavior: DeleteBehavior::FailsWithUnexpected(message),
            ..Self::that_returns_mac_address(mac_address)
        }
    }

    pub fn that_finds_nothing() -> Self {
        Self::with(SaveBehavior::Succeeds, FindByIdBehavior::ReturnsNone)
    }

    pub fn that_fails_on_find(message: String) -> Self {
        Self::with(
            SaveBehavior::Succeeds,
            FindByIdBehavior::FailsWithUnexpected(message),
        )
    }

    fn with(save_behavior: SaveBehavior, find_by_id_behavior: FindByIdBehavior) -> Self {
        Self {
            save_behavior,
            find_by_id_behavior,
            delete_behavior: DeleteBehavior::Succeeds,
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
impl MacAddressRepository for MacAddressRepositoryMock {
    async fn save(&self, mac_address: &MacAddress) -> Result<(), MacAddressRepositoryError> {
        match &self.save_behavior {
            SaveBehavior::FailsWithAlreadyExists => Err(MacAddressRepositoryError::AlreadyExists),
            SaveBehavior::FailsWithUnexpected(message) => {
                Err(MacAddressRepositoryError::Unexpected(message.clone()))
            }
            SaveBehavior::Succeeds => {
                self.saved_ids
                    .lock()
                    .unwrap()
                    .push(*mac_address.id().value());
                Ok(())
            }
        }
    }

    async fn find_by_id(
        &self,
        _id: &MacAddressId,
    ) -> Result<Option<MacAddress>, MacAddressRepositoryError> {
        match &self.find_by_id_behavior {
            FindByIdBehavior::ReturnsNone => Ok(None),
            FindByIdBehavior::ReturnsMacAddress(cell) => Ok(cell.lock().unwrap().take()),
            FindByIdBehavior::FailsWithUnexpected(message) => {
                Err(MacAddressRepositoryError::Unexpected(message.clone()))
            }
        }
    }

    async fn delete(&self, _id: &MacAddressId) -> Result<(), MacAddressRepositoryError> {
        *self.delete_call_count.lock().unwrap() += 1;
        match &self.delete_behavior {
            DeleteBehavior::Succeeds => Ok(()),
            DeleteBehavior::FailsWithUnexpected(message) => {
                Err(MacAddressRepositoryError::Unexpected(message.clone()))
            }
        }
    }
}
