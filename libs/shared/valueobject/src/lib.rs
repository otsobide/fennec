//! # Value Objects
//!
//! This crate provides reusable Value Object implementations following
//! Domain-Driven Design (DDD) principles.
//!
//! A Value Object is an immutable object defined by its value rather than
//! its identity. Validation is enforced at construction time to guarantee
//! domain invariants.
//!
//! ## Main modules
//!
//! - [`domain`]: canonical JSON, DNS names, STIX identifiers, timestamps,
//!   strings and the validation error every value object returns.
//! - [`application`]: (Reserved for future use.)
//! - [`infrastructure`]: (Reserved for future use.)
//!
//! ## Usage example
//!
//! ```rust
//! use shared_valueobject::domain::strings::string_value_object::StringValueObject;
//!
//! let name = StringValueObject::new("alice".to_string());
//! assert_eq!(name.value(), "alice");
//! ```

pub mod application;
pub mod domain;
pub mod infrastructure;
