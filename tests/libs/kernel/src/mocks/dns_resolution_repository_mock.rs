use std::sync::Mutex;

use async_trait::async_trait;

use kernel::dns_resolution::domain::entities::dns_resolution::DnsResolution;
use kernel::dns_resolution::domain::errors::dns_resolution_repository_error::DnsResolutionRepositoryError;
use kernel::dns_resolution::domain::repositories::dns_resolution_repository::DnsResolutionRepository;
use kernel::dns_resolution::domain::value_objects::dns_resolution_answer::DnsResolutionAnswer;
use kernel::dns_resolution::domain::value_objects::dns_resolution_id::DnsResolutionId;
use kernel::dns_resolution::domain::value_objects::dns_resolution_subject::DnsResolutionSubject;

#[allow(dead_code)]
pub enum SaveBehavior {
    Succeeds,
    FailsWithAlreadyExists,
    FailsWithUnexpected(String),
}

#[allow(dead_code)]
pub enum UpdateBehavior {
    Succeeds,
    FailsWithNotFound,
    FailsWithUnexpected(String),
}

/// `ReturnsDnsResolution` answers only a lookup of that resolution's id, so a
/// service that derives the wrong id finds nothing.
#[allow(dead_code)]
pub enum FindByIdBehavior {
    ReturnsNone,
    ReturnsDnsResolution(DnsResolution),
    FailsWithUnexpected(String),
}

/// `ReturnsMatching` returns, in the given order, the resolutions whose
/// subject is the queried one, at most `limit` of them.
#[allow(dead_code)]
pub enum ListBySubjectBehavior {
    ReturnsMatching(Vec<DnsResolution>),
    FailsWithUnexpected(String),
}

/// `ReturnsMatching` returns, in the given order, the resolutions whose
/// answer is the queried one, at most `limit` of them.
#[allow(dead_code)]
pub enum ListByAnswerBehavior {
    ReturnsMatching(Vec<DnsResolution>),
    FailsWithUnexpected(String),
}

#[allow(dead_code)]
pub enum DeleteBehavior {
    Succeeds,
    FailsWithNotFound,
    FailsWithUnexpected(String),
}

pub struct DnsResolutionRepositoryMock {
    save_behavior: SaveBehavior,
    update_behavior: UpdateBehavior,
    find_by_id_behavior: FindByIdBehavior,
    list_by_subject_behavior: ListBySubjectBehavior,
    list_by_answer_behavior: ListByAnswerBehavior,
    delete_behavior: DeleteBehavior,
    saved: Mutex<Vec<DnsResolution>>,
    updated: Mutex<Vec<DnsResolution>>,
    found_ids: Mutex<Vec<DnsResolutionId>>,
    listed_by_subject: Mutex<Vec<(DnsResolutionSubject, usize)>>,
    listed_by_answer: Mutex<Vec<(DnsResolutionAnswer, usize)>>,
    deleted_ids: Mutex<Vec<DnsResolutionId>>,
}

#[allow(dead_code)]
impl DnsResolutionRepositoryMock {
    fn base() -> Self {
        Self {
            save_behavior: SaveBehavior::Succeeds,
            update_behavior: UpdateBehavior::Succeeds,
            find_by_id_behavior: FindByIdBehavior::ReturnsNone,
            list_by_subject_behavior: ListBySubjectBehavior::ReturnsMatching(Vec::new()),
            list_by_answer_behavior: ListByAnswerBehavior::ReturnsMatching(Vec::new()),
            delete_behavior: DeleteBehavior::Succeeds,
            saved: Mutex::new(vec![]),
            updated: Mutex::new(vec![]),
            found_ids: Mutex::new(vec![]),
            listed_by_subject: Mutex::new(vec![]),
            listed_by_answer: Mutex::new(vec![]),
            deleted_ids: Mutex::new(vec![]),
        }
    }

    /// Every write succeeds, nothing is recorded and every list is empty.
    pub fn that_succeeds() -> Self {
        Self::base()
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

    pub fn that_fails_with_already_exists() -> Self {
        Self {
            save_behavior: SaveBehavior::FailsWithAlreadyExists,
            ..Self::base()
        }
    }

    pub fn that_fails_on_save(message: String) -> Self {
        Self {
            save_behavior: SaveBehavior::FailsWithUnexpected(message),
            ..Self::base()
        }
    }

    pub fn that_returns_dns_resolution(dns_resolution: DnsResolution) -> Self {
        Self {
            find_by_id_behavior: FindByIdBehavior::ReturnsDnsResolution(dns_resolution),
            ..Self::base()
        }
    }

    pub fn that_returns_dns_resolution_but_update_fails(
        dns_resolution: DnsResolution,
        message: String,
    ) -> Self {
        Self {
            update_behavior: UpdateBehavior::FailsWithUnexpected(message),
            ..Self::that_returns_dns_resolution(dns_resolution)
        }
    }

    pub fn that_returns_dns_resolution_but_delete_fails(
        dns_resolution: DnsResolution,
        message: String,
    ) -> Self {
        Self {
            delete_behavior: DeleteBehavior::FailsWithUnexpected(message),
            ..Self::that_returns_dns_resolution(dns_resolution)
        }
    }

    /// Both lists answer from `dns_resolutions`, filtered by the queried
    /// subject or answer.
    pub fn that_lists(dns_resolutions: Vec<DnsResolution>) -> Self {
        Self {
            list_by_subject_behavior: ListBySubjectBehavior::ReturnsMatching(
                dns_resolutions.clone(),
            ),
            list_by_answer_behavior: ListByAnswerBehavior::ReturnsMatching(dns_resolutions),
            ..Self::base()
        }
    }

    pub fn that_fails_on_list_by_subject(message: String) -> Self {
        Self {
            list_by_subject_behavior: ListBySubjectBehavior::FailsWithUnexpected(message),
            ..Self::base()
        }
    }

    /// `list_by_subject` answers from `dns_resolutions`; `list_by_answer`
    /// fails.
    pub fn that_lists_but_list_by_answer_fails(
        dns_resolutions: Vec<DnsResolution>,
        message: String,
    ) -> Self {
        Self {
            list_by_answer_behavior: ListByAnswerBehavior::FailsWithUnexpected(message),
            ..Self::that_lists(dns_resolutions)
        }
    }

    pub fn saved(&self) -> Vec<DnsResolution> {
        self.saved.lock().unwrap().clone()
    }

    pub fn updated(&self) -> Vec<DnsResolution> {
        self.updated.lock().unwrap().clone()
    }

    pub fn found_ids(&self) -> Vec<DnsResolutionId> {
        self.found_ids.lock().unwrap().clone()
    }

    pub fn listed_by_subject(&self) -> Vec<(DnsResolutionSubject, usize)> {
        self.listed_by_subject.lock().unwrap().clone()
    }

    pub fn listed_by_answer(&self) -> Vec<(DnsResolutionAnswer, usize)> {
        self.listed_by_answer.lock().unwrap().clone()
    }

    pub fn deleted_ids(&self) -> Vec<DnsResolutionId> {
        self.deleted_ids.lock().unwrap().clone()
    }
}

#[async_trait]
impl DnsResolutionRepository for DnsResolutionRepositoryMock {
    async fn save(
        &self,
        dns_resolution: &DnsResolution,
    ) -> Result<(), DnsResolutionRepositoryError> {
        match &self.save_behavior {
            SaveBehavior::Succeeds => {
                self.saved.lock().unwrap().push(dns_resolution.clone());
                Ok(())
            }
            SaveBehavior::FailsWithAlreadyExists => {
                Err(DnsResolutionRepositoryError::AlreadyExists)
            }
            SaveBehavior::FailsWithUnexpected(message) => {
                Err(DnsResolutionRepositoryError::Unexpected(message.clone()))
            }
        }
    }

    async fn update(
        &self,
        dns_resolution: &DnsResolution,
    ) -> Result<(), DnsResolutionRepositoryError> {
        match &self.update_behavior {
            UpdateBehavior::Succeeds => {
                self.updated.lock().unwrap().push(dns_resolution.clone());
                Ok(())
            }
            UpdateBehavior::FailsWithNotFound => Err(DnsResolutionRepositoryError::NotFound),
            UpdateBehavior::FailsWithUnexpected(message) => {
                Err(DnsResolutionRepositoryError::Unexpected(message.clone()))
            }
        }
    }

    async fn find_by_id(
        &self,
        id: &DnsResolutionId,
    ) -> Result<Option<DnsResolution>, DnsResolutionRepositoryError> {
        self.found_ids.lock().unwrap().push(*id);
        match &self.find_by_id_behavior {
            FindByIdBehavior::ReturnsNone => Ok(None),
            FindByIdBehavior::ReturnsDnsResolution(dns_resolution) => {
                Ok(Some(dns_resolution.clone()).filter(|found| found.id() == id))
            }
            FindByIdBehavior::FailsWithUnexpected(message) => {
                Err(DnsResolutionRepositoryError::Unexpected(message.clone()))
            }
        }
    }

    async fn list_by_subject(
        &self,
        subject: &DnsResolutionSubject,
        limit: usize,
    ) -> Result<Vec<DnsResolution>, DnsResolutionRepositoryError> {
        self.listed_by_subject
            .lock()
            .unwrap()
            .push((subject.clone(), limit));
        match &self.list_by_subject_behavior {
            ListBySubjectBehavior::ReturnsMatching(dns_resolutions) => Ok(dns_resolutions
                .iter()
                .filter(|dns_resolution| dns_resolution.subject() == subject)
                .take(limit)
                .cloned()
                .collect()),
            ListBySubjectBehavior::FailsWithUnexpected(message) => {
                Err(DnsResolutionRepositoryError::Unexpected(message.clone()))
            }
        }
    }

    async fn list_by_answer(
        &self,
        answer: &DnsResolutionAnswer,
        limit: usize,
    ) -> Result<Vec<DnsResolution>, DnsResolutionRepositoryError> {
        self.listed_by_answer.lock().unwrap().push((*answer, limit));
        match &self.list_by_answer_behavior {
            ListByAnswerBehavior::ReturnsMatching(dns_resolutions) => Ok(dns_resolutions
                .iter()
                .filter(|dns_resolution| dns_resolution.answer() == answer)
                .take(limit)
                .cloned()
                .collect()),
            ListByAnswerBehavior::FailsWithUnexpected(message) => {
                Err(DnsResolutionRepositoryError::Unexpected(message.clone()))
            }
        }
    }

    async fn delete(&self, id: &DnsResolutionId) -> Result<(), DnsResolutionRepositoryError> {
        match &self.delete_behavior {
            DeleteBehavior::Succeeds => {
                self.deleted_ids.lock().unwrap().push(*id);
                Ok(())
            }
            DeleteBehavior::FailsWithNotFound => Err(DnsResolutionRepositoryError::NotFound),
            DeleteBehavior::FailsWithUnexpected(message) => {
                Err(DnsResolutionRepositoryError::Unexpected(message.clone()))
            }
        }
    }
}
