//! This module mostly contains structs pertaining to web requests.
//! Currently only encludes [BodyDeserializeError].

use std::{fmt, borrow::Cow};
use std::error::Error as StdError;

use chrono::NaiveTime;
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

use crate::graphql::resolvers::PronounSet;
use crate::database::table_schemas::Teachers::TeacherPresence::TeacherPresence;
use tokio_postgres::Row;
use const_format::formatcp;


/// The direct deserialization target of a `Teachers` table row.
pub struct TeacherRow {
    /// The id in the `teacherid` field.
    pub id: TeacherId,
    
    /// The value in the `firstname` field.
    pub first_name: String,

    /// The value in the `lastname` field.
    pub last_name: String,

    /// A combination of the `isabsent` and `fullyabsent` fields conglomerated into 1 enum.
    pub presence: TeacherPresence,

    pub pronoun_set: PronounSet,

    pub honorific: String,
}

impl TryFrom<Row> for TeacherRow {
    type Error = Cow<'static, str>;
    fn try_from(row: Row) -> Result<Self, Self::Error> {
        /// FIXME: Centralize this constant.
        const COL_NAMES: [&str; 7] = [
            "teacherid",
            "firstname", "lastname",
            "honorific", "pronouns",
            "isabsent", "fullyabsent",
        ];


        match (
            row.try_get(COL_NAMES[0]),
            row.try_get(COL_NAMES[1]),
            row.try_get(COL_NAMES[2]),
        ) {
            (
                Ok(id),
                Ok(first_name),
                Ok(last_name),
            ) => Ok(Self {
                id: TeacherId::new(&id),
                first_name,
                last_name,
                presence: match (row.try_get(COL_NAMES[5]), row.try_get(COL_NAMES[6])) {
                    (Ok(_), Ok(true)) => TeacherPresence::FullAbsent,
                    (Ok(true), Ok(false)) => TeacherPresence::PartAbsent,
                    (Ok(false), Ok(false)) => TeacherPresence::FullPresent,
                    _ => return Err("Row does not contain valid absence state".into()),
                },
                honorific: match row.try_get(COL_NAMES[3]) {
                    Ok(string) => string,
                    Err(_) => return Err(formatcp!("Row does not contain {:?}", COL_NAMES[3]).into()),
                },
                pronoun_set: match row.try_get(COL_NAMES[4]).map(|db_string: String| PronounSet::try_new(&db_string)) {
                    Ok(Ok(set_input)) => set_input,
                    Ok(Err(_)) => return Err("Row does not contain valid pronoun set".into()),
                    Err(_) => return Err(formatcp!("Row does not contain {:?}", COL_NAMES[4]).into()),
                },
            }),
            (id, first, last) => {
                let mut formatted = "Row does not contain ".to_string();
                if id.is_err() {
                    formatted.push_str(COL_NAMES[0]);
                    formatted.push_str(", ");
                }
                if first.is_err() {
                    formatted.push_str(COL_NAMES[1]);
                    formatted.push_str(", ");
                }
                if last.is_err() {
                    formatted.push_str(COL_NAMES[2]);
                    formatted.push_str(", ");
                }
                formatted.truncate(formatted.len() - 2);
                Err(formatted.into())
            }
        }
    }
}

/// The direct deserialization target of a `Teachers` table row.
pub struct PeriodRow {
    /// The id in the `periodid` field.
    pub id: PeriodId,
    
    /// The name in the `periodname` field.
    pub name: PeriodName,

    pub start_default: NaiveTime,
    pub end_default: NaiveTime,
    
    pub start_curr: Option<NaiveTime>,
    pub end_curr: Option<NaiveTime>,
}

impl TryFrom<Row> for PeriodRow {
    type Error = Cow<'static, str>;
    fn try_from(row: Row) -> Result<Self, Self::Error> {
        /// FIXME: Centralize this constant.
        const COL_NAMES: [&str; 6] = [
            "PeriodId", "PeriodName",
            "UtcStartDefault", "UtcEndDefault",
            "UtcStartCurrent", "UtcEndCurrent",
        ];

        match (
            row.try_get(COL_NAMES[0]), row.try_get(COL_NAMES[1]),
            row.try_get(COL_NAMES[2]), row.try_get(COL_NAMES[3]),
            row.try_get(COL_NAMES[4]), row.try_get(COL_NAMES[5]),
        ) {
            (
                Ok(id), Ok(name),
                Ok(start_default), Ok(end_default),
                Ok(start_curr), Ok(end_curr),
            ) => Ok(PeriodRow {
                id: PeriodId::new(&id),
                name: PeriodName::new(name), 
                start_default, end_default, start_curr, end_curr,
            }),
            e => Err(format!("SQL error: {:?}", e).into()),
        }

    }
}
