//! # PostgreSQL persistence
//!
//! Generic PostgreSQL infrastructure shared by every bounded context that
//! persists in PostgreSQL: the event store schema and its migrations, the
//! unit of work that makes a command's state change and its domain events
//! commit together, the outbox event bus and the transactional command bus
//! decorator. It contains no CTI logic.
//!
//! The design is described in `docs/architecture/persistence.md` and
//! `docs/architecture/event-store.md`.

pub mod infrastructure;
