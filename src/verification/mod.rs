//! This module contains most of the relevant functions and utilities for verifying the validity of a request.
//! Most importantly, it contains
//! - [AuthenticationMethods], a struct representing authentication status,
//! - [auth_all_method_gen], a function returning a filter which generates an `AuthenticationMethods` struct for each request
//! 
//! It may be eventually prudent to reimplement the filters to only run when for performance reasons.
//! TODO: Profile this.

mod secret_map;
pub mod hmac;

use warp::{Filter, Rejection};

use crate::preludes::macros::byte_vec_wrapper;

byte_vec_wrapper!{ /** A wrapper for a request body ("Payload"). */ Payload }
byte_vec_wrapper!{ /** A wrapper for a request's hmac signing data ("Signature"). */ Signature }
byte_vec_wrapper!{ /** A wrapper for the bytes of a client's key data ("Secret"). */ Secret }

/// A struct containing an indicator of status for all of the available methods of authentication (currently only rolling HMAC)
#[derive(Debug, Clone, Copy)]
pub struct AuthenticationMethods {
    /// The request validity for rolling HMAC auth.
    pub hmac: bool,
}

/// Returns a filter which outputs a tuple containing the auth method status and the owned request body.
pub fn auth_all_method_gen() -> impl Filter<Extract = ((AuthenticationMethods, warp::hyper::body::Bytes),), Error = Rejection> + Clone {
    self::hmac::hmac_verify_filter()
        .untuple_one()
        .map(|hmac, bytes| (
            AuthenticationMethods { hmac },
            bytes,
        ))
}
