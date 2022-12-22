//! This module mostly contains structs pertaining to web requests.
//! Currently only encludes [BodyDeserializeError].

use std::fmt;
use std::error::Error as StdError;

use warp::reject::Reject;

/// A general error type, currently only used for [BodyDeserializeError].
pub type BoxError = Box<dyn StdError + Send + Sync>;

/// Represents an error encountered because of an inability to deserialize bytes, whether internal or external.
pub struct BodyDeserializeError {
    /// Dynamic thread-safe boxed std::error::Error.
    cause: BoxError,
}

impl BodyDeserializeError {
    /// Creates a new `BodyDeserializeError` with the given cause.
    pub fn from_cause(cause: BoxError) -> Self {
        Self { cause }
    }
}

impl fmt::Display for BodyDeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Request body deserialize error: {}", self.cause)
    }
}

impl fmt::Debug for BodyDeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl StdError for BodyDeserializeError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(self.cause.as_ref())
    }
}


impl Reject for BodyDeserializeError {}
