// #![warn(clippy::missing_docs_in_private_items)]

// TODO: Document Better

//! ## IMPORTANT
//! ### This library should only be used within the crate `improved-eureka`, for the backend system `improved-eureka`.
//! 
//! ## Description
//! `improved-eureka` is the main backend and API for the YBT (working name) system.
//! `improved-eureka` is the system's "source-of-truth", providing a GraphQL API, authentication, and inter-microservice communication.
//! 
//! ## Duties
//! It communicates with IOT devices and microservices including:
//! - admin-side input system 
//! - student-side scan-in system
//! - notif system
//!     - student scan-in confirmation system
//!     - teacher absence notif system
//! - school server data mirroring system
//! 
//! ## Considerations
//! This server is the most critical single point of failure for the entire system.
//! Given that, as a note to our developers:
//! - Panics and unwraps are NOT ACCEPTABLE in _production_ code. Check for them before merging to trunk.
//!     - Try to avoid them before making pull requests.
//!     - Use command `RUSTFLAGS="-Dclippy::unwrap_used -Dclippy::unwrap_in_result" cargo check --release` to ensure no unwraps or expects are used.
//! - _(as soon as i implement it)_ Please use the logging system to log infrequent or server-relevant info.
//!     - You don't need to record _everything_, but errors, rejections, rate-limiting, especially
//! - Document your stuff please.
//!     - It's not a hard error, but fix it before merging or pull requests.
//! 
//! ## Crate Layout
//! This crate is intended to be separated into the main components and modular duties of `eureka`. These include:
//! - [database], for interacting with the persistent local SQL database
//! - [graphql], for handling deserialized graphql queries
//! - [verification], for precalculating diferent methods of verification to allow for greater security
//! 
//! Although this library's modules are public, most of the most important parts of each can be accessed from the [preludes] module.
//! 
//! ## Things Left to Do
//! - Outward communication for notifs
//!     - Redis?
//!     - `POST` request to separate web server?
//! - Mirroring to the school server
//!     - Internal?
//!     - Redis?
//!     - `POST` requests to separate web server?

pub mod utils;
pub mod preludes;

pub mod graphql;
pub mod database;

pub mod verification;

pub use preludes::*;
