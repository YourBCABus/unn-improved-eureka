//! This module contains most of the relevant functions and utilities for verifying the validity of a request.
//! Most importantly, it contains
//! - [`AuthenticationMethods`], a struct representing authentication status,
//! - [`auth_all_method_gen`], a function returning a filter which generates an `AuthenticationMethods` struct for each request
//! 
//! It may be eventually prudent to reimplement the filters to only run when for performance reasons.
//! TODO: Profile this.

use actix_web::http::header::{Header, TryIntoHeaderValue};
use actix_web::error::ParseError;

use reqwest::header::{HeaderValue, InvalidHeaderValue, HeaderName};
use uuid::Uuid;

pub mod id_secret;
pub mod scopes;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientSecretHeader(Vec<u8>);

impl ClientSecretHeader {
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Header for ClientSecretHeader {
    fn name() -> HeaderName {
        HeaderName::from_static("client-secret")
    }
    fn parse<M: actix_web::HttpMessage>(msg: &M) -> Result<Self, ParseError> {
        let Some(header) = msg.headers().get(Self::name()) else {
            return Err(ParseError::Header);
        };

        Ok(Self(header.as_bytes().to_vec()))
    }
}

impl TryIntoHeaderValue for ClientSecretHeader {
    type Error = InvalidHeaderValue;
    fn try_into_value(self) -> Result<HeaderValue, Self::Error> {
        HeaderValue::from_bytes(&self.0)
    }
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientIdHeader(Uuid);

impl ClientIdHeader {
    pub fn inner(&self) -> Uuid {
        self.0
    }
}

impl Header for ClientIdHeader {
    fn name() -> HeaderName {
        HeaderName::from_static("client-id")
    }
    fn parse<M: actix_web::HttpMessage>(msg: &M) -> Result<Self, ParseError> {
        let Some(header) = msg.headers().get(Self::name()) else {
            return Err(ParseError::Header);
        };

        let Ok(id) = header.to_str() else {
            return Err(ParseError::Header);
        };

        let Ok(id) = uuid::Uuid::parse_str(id) else {
            return Err(ParseError::Header);
        };

        Ok(Self(id))
    }
}

impl TryIntoHeaderValue for ClientIdHeader {
    type Error = InvalidHeaderValue;
    fn try_into_value(self) -> Result<HeaderValue, Self::Error> {
        HeaderValue::from_str(&self.0.hyphenated().to_string())
    }
}
