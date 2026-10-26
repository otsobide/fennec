//! Query for finding a single autonomous system.

use shared_cqrs::query::domain::query::Query;

use crate::autonomous_system::domain::value_objects::autonomous_system_id::AutonomousSystemId;

/// Query that requests the autonomous system with the given id.
///
/// A lookup by value builds the id from the value first, so it is the same
/// query.
pub struct FindAutonomousSystemQuery {
    pub id: AutonomousSystemId,
}

impl Query for FindAutonomousSystemQuery {}
