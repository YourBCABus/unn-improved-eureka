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

use crate::{
    graphql_types::teachers::{TeacherId, TeacherName},
    database::table_schemas::Teachers::TeacherPresence::TeacherPresence,
};
use tokio_postgres::Row;

pub struct TeacherRow {
    pub id: TeacherId,
    pub name: TeacherName,
    pub presence: TeacherPresence,
}

impl TryFrom<Row> for TeacherRow {
    type Error = String;
    fn try_from(row: Row) -> Result<Self, Self::Error> {
        /// FIXME: Centralize this constant.
        const COL_NAMES: [&str; 4] = ["teacherid", "teachername", "isabsent", "fullyabsent"];

        match (row.try_get(COL_NAMES[0]), row.try_get(COL_NAMES[1])) {
            (Ok(id), Ok(name)) => Ok(Self {
                id: TeacherId::new(&id),
                name: TeacherName::new(name), 
                presence: match (row.try_get(COL_NAMES[2]), row.try_get(COL_NAMES[3])) {
                    (Ok(_), Ok(true)) => TeacherPresence::FullAbsent,
                    (Ok(true), Ok(false)) => TeacherPresence::PartAbsent,
                    (Ok(false), Ok(false)) => TeacherPresence::FullPresent,
                    (a, b) => Err(format!("Row does not contain valid absence state: {:?}, {:?}.", a, b))?,
                },
            }),
            (Ok(_), Err(_)) => Err(format!("Row does not contain {:?}", [COL_NAMES[1]])),
            (Err(_), Ok(_)) => Err(format!("Row does not contain {:?}", [COL_NAMES[0]])),
            (Err(_), Err(_)) => Err(format!("Row does not contain {:?}", [COL_NAMES[0], COL_NAMES[1]])),
        }

    }
}