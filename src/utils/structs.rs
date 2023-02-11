//! This module mostly contains structs pertaining to web requests.
//! Currently only encludes [BodyDeserializeError].

use std::{fmt, borrow::Cow};
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
    graphql_types::scalars::{
        teacher::*,
        period::*,
    },
    database::table_schemas::Teachers::TeacherPresence::TeacherPresence,
};
use tokio_postgres::Row;
use const_format::formatcp;


/// The direct deserialization target of a `Teachers` table row.
pub struct TeacherRow {
    /// The id in the `teacherid` field.
    pub id: TeacherId,
    
    /// The name in the `teachername` field.
    pub name: TeacherName,

    /// A combination of the `isabsent` and `fullyabsent` fields conglomerated into 1 enum.
    pub presence: TeacherPresence,
}

impl TryFrom<Row> for TeacherRow {
    type Error = Cow<'static, str>;
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
                    _ => Err(Cow::from("Row does not contain valid absence state"))?,
                },
            }),
            (Ok(_), Err(_)) => Err(formatcp!("Row does not contain {:?}", COL_NAMES[1]).into()),
            (Err(_), Ok(_)) => Err(formatcp!("Row does not contain {:?}", COL_NAMES[0]).into()),
            (Err(_), Err(_)) => Err(formatcp!("Row does not contain {:?}, {:?}", COL_NAMES[0], COL_NAMES[1]).into()),
        }
    }
}

/// The direct deserialization target of a `Teachers` table row.
pub struct PeriodRow {
    /// The id in the `periodid` field.
    pub id: PeriodId,
    
    /// The name in the `periodname` field.
    pub name: PeriodName,
}

impl TryFrom<Row> for PeriodRow {
    type Error = Cow<'static, str>;
    fn try_from(row: Row) -> Result<Self, Self::Error> {
        /// FIXME: Centralize this constant.
        const COL_NAMES: [&str; 2] = ["periodid", "periodname"];

        match (row.try_get(COL_NAMES[0]), row.try_get(COL_NAMES[1])) {
            (Ok(id), Ok(name)) => Ok(PeriodRow {
                id: PeriodId::new(&id),
                name: PeriodName::new(name), 
            }),
            (Ok(_), Err(_)) => Err(formatcp!("Row does not contain {:?}", COL_NAMES[1]).into()),
            (Err(_), Ok(_)) => Err(formatcp!("Row does not contain {:?}", COL_NAMES[0]).into()),
            (Err(_), Err(_)) => Err(formatcp!("Row does not contain {:?}, {:?}", COL_NAMES[0], COL_NAMES[1]).into()),
        }

    }
}
