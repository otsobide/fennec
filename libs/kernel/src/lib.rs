//! # Kernel
//!
//! The CTI kernel bounded context. It hosts one module per aggregate, each with
//! its own full `domain` / `application` / `infrastructure` tree.
//!
//! The modules are **isolated from each other by design**: none of them imports
//! another's types. They relate by shared identifier (each module declares its
//! own value object for a foreign id) and communicate only through the command,
//! query and event buses of `libs/shared/`. Promoting a module to its own crate
//! is therefore a move, not a redesign.

pub mod domain_name;
pub mod email_address;
pub mod hostname;
pub mod ipv4_address;
pub mod ipv6_address;
pub mod kernel_migrator;
pub mod sighting;
pub mod source;
pub mod url_source;
