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
//! - [`database`], for interacting with the persistent local SQL database
//! - [`graphql`], for handling deserialized graphql queries
//! - Smaller parts, including:
//!     - [`types`] for general types to represent internal "objects" (Teachers,
//!       Periods, Absences, etc)
//!     - [`state`] for a way to globally store the Schema and the database
//!       connection pool
//!     - [`logs_env::logging`] for all logging in the crate
//!     - [`logs_env::env`] for pre-checking all of the environment variables on
//!       server startup
//! 
//! 
//! ## Things Left to Do
//! - Outward communication for notifs
//!     - Redis?
//!     - `POST` request to separate web server?
//! - Mirroring to the school server
//!     - Internal?
//!     - Redis?
//!     - `POST` requests to separate web server?
//! - ***Auth!***

pub mod graphql;
pub mod database;

pub mod notifications;

// pub mod verification;




pub mod types;
pub mod state;
pub mod logs_env;
pub use logs_env::*;