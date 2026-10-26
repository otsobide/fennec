use std::sync::Mutex;

use async_trait::async_trait;
use uuid::Uuid;

use kernel::email_address::domain::entities::email_address::EmailAddress;
use kernel::email_address::domain::errors::email_address_repository_error::EmailAddressRepositoryError;
use kernel::email_address::domain::repositories::email_address_repository::EmailAddressRepository;
use kernel::email_address::domain::value_objects::email_address_id::EmailAddressId;

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
    ReturnsEmailAddress(Mutex<Option<EmailAddress>>),
    FailsWithUnexpected(String),
}

pub struct EmailAddressRepositoryMock {
    save_behavior: SaveBehavior,
    find_by_id_behavior: FindByIdBehavior,
    delete_behavior: DeleteBehavior,
    saved_ids: Mutex<Vec<Uuid>>,
    delete_call_count: Mutex<u32>,
}

#[allow(dead_code)]
impl EmailAddressRepositoryMock {
    pub fn that_succeeds() -> Self {
        Self::with(SaveBehavior::Succeeds, FindByIdBehavior::ReturnsNone)
    }

    pub fn that_fails_with_already_exists() -> Self {
        Self::with(
            SaveBehavior::FailsWithAlreadyExists,
            FindByIdBehavior::ReturnsNone,
        )
    }

    pub fn that_returns_email_address(email_address: EmailAddress) -> Self {
        Self::with(
            SaveBehavior::Succeeds,
            FindByIdBehavior::ReturnsEmailAddress(Mutex::new(Some(email_address))),
        )
    }

    pub fn that_fails_on_save(message: String) -> Self {
        Self::with(
            SaveBehavior::FailsWithUnexpected(message),
            FindByIdBehavior::ReturnsNone,
        )
    }

    pub fn that_returns_email_address_but_delete_fails(
        email_address: EmailAddress,
        message: String,
    ) -> Self {
        Self {
            delete_behavior: DeleteBehavior::FailsWithUnexpected(message),
            ..Self::that_returns_email_address(email_address)
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
impl EmailAddressRepository for EmailAddressRepositoryMock {
    async fn save(&self, email_address: &EmailAddress) -> Result<(), EmailAddressRepositoryError> {
        match &self.save_behavior {
            SaveBehavior::FailsWithAlreadyExists => Err(EmailAddressRepositoryError::AlreadyExists),
            SaveBehavior::FailsWithUnexpected(message) => {
                Err(EmailAddressRepositoryError::Unexpected(message.clone()))
            }
            SaveBehavior::Succeeds => {
                self.saved_ids
                    .lock()
                    .unwrap()
                    .push(*email_address.id().value());
                Ok(())
            }
        }
    }

    async fn find_by_id(
        &self,
        _id: &EmailAddressId,
    ) -> Result<Option<EmailAddress>, EmailAddressRepositoryError> {
        match &self.find_by_id_behavior {
            FindByIdBehavior::ReturnsNone => Ok(None),
            FindByIdBehavior::ReturnsEmailAddress(cell) => Ok(cell.lock().unwrap().take()),
            FindByIdBehavior::FailsWithUnexpected(message) => {
                Err(EmailAddressRepositoryError::Unexpected(message.clone()))
            }
        }
    }

    async fn delete(&self, _id: &EmailAddressId) -> Result<(), EmailAddressRepositoryError> {
        *self.delete_call_count.lock().unwrap() += 1;
        match &self.delete_behavior {
            DeleteBehavior::Succeeds => Ok(()),
            DeleteBehavior::FailsWithUnexpected(message) => {
                Err(EmailAddressRepositoryError::Unexpected(message.clone()))
            }
        }
    }
}
