//! This module contains most of the relevant functions and utilities for verifying the validity of a request.
//! Most importantly, it contains
//! - [`AuthenticationMethods`], a struct representing authentication status,
//! - [`auth_all_method_gen`], a function returning a filter which generates an `AuthenticationMethods` struct for each request
//! 
//! It may be eventually prudent to reimplement the filters to only run when for performance reasons.
//! TODO: Profile this.

pub mod client;
pub mod user;

pub mod context;

pub mod scopes;
pub mod headers;

pub use headers::*;
