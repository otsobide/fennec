//! `Ioc` aggregate root.
//!
//! Represents an Indicator of Compromise: a single observable (IP, domain,
//! URL, hash, email, ...) that is considered relevant for detection or
//! investigation.

use crate::ioc::domain::value_objects::ioc_created_at::IocCreatedAt;
use crate::ioc::domain::value_objects::ioc_id::IocId;
use crate::ioc::domain::value_objects::ioc_type::IocType;
use crate::ioc::domain::value_objects::ioc_updated_at::IocUpdatedAt;
use crate::ioc::domain::value_objects::ioc_value::IocValue;

/// Aggregate root modelling an Indicator of Compromise.
#[derive(Clone)]
pub struct Ioc {
    id: IocId,
    ioc_type: IocType,
    value: IocValue,
    created_at: IocCreatedAt,
    updated_at: IocUpdatedAt,
}

impl Ioc {
    /// Creates a new `Ioc` from its component value objects.
    pub fn new(
        id: IocId,
        ioc_type: IocType,
        value: IocValue,
        created_at: IocCreatedAt,
        updated_at: IocUpdatedAt,
    ) -> Self {
        Self {
            id,
            ioc_type,
            value,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> &IocId {
        &self.id
    }

    pub fn ioc_type(&self) -> &IocType {
        &self.ioc_type
    }

    pub fn value(&self) -> &IocValue {
        &self.value
    }

    pub fn created_at(&self) -> &IocCreatedAt {
        &self.created_at
    }

    pub fn updated_at(&self) -> &IocUpdatedAt {
        &self.updated_at
    }
}
