
use std::fmt::Display;

use super::*;
use super::pronoun_set::PronounSet;
use absence_state::AbsenceStateMetadata;
use juniper::graphql_object;

use crate::graphql_types::{
    scalars::teacher::*,
    *,
};
use crate::database::prelude::{TeacherRow, TeacherPresence};



/// This struct represents a Teacher with no period information associated with the absence_state.
/// 
/// This is mainly used to be an graphql-compatible type for [TeacherRow],
/// and usually should be created with [From] or [Into].
/// 
/// [juniper] will apply resolvers to it to get the required fields.
#[derive(Debug, Clone)]
pub struct TeacherMetadata {
    /// The id of the teacher. This is essentially a wrapper for a UUID, but is (de)serializable for juniper.
    pub id: TeacherId,
    /// The name of the teacher. A String wrapper.
    pub name: TeacherName,

    /// The honorific of a the teacher.
    /// 
    /// Examples: `Mr.`, `Ms.`, `Mx.`, `Dr.`, etc.
    pub honorific: String,

    /// The pronouns of the teacher. Contains:
    /// - The entire set of `[sub, obj, posadj, pospro, ref]`.
    /// - Grammatical plurality information. (Like `he is` vs `they are`)
    /// (This model supports neopronouns.)
    /// 
    /// Examples:
    /// - `[ she,  her,   her,   hers,  herself], false`
    /// - `[  he,  him,   his,    his,  himself], false`
    /// - `[they, them, their, theirs, themself], true`
    /// - `[  xe,  xem,   xyr,   xyrs,  xemself], false`
    pub pronouns: PronounSet,
    
    /// The stripped absence state of the teacher.
    pub absence_state_meta: TeacherPresence,
}

impl From<TeacherRow> for TeacherMetadata {
    fn from(row: TeacherRow) -> Self {
        Self {
            id: row.id,
            name: row.name,
            honorific: row.honorific,
            pronouns: row.pronoun_set,
            absence_state_meta: row.presence,
        }
    }
}

type TryFromError = <TeacherRow as TryFrom<tokio_postgres::Row>>::Error;
impl TeacherMetadata {
    pub fn try_from_row<T>(row: tokio_postgres::Row, err_map: impl FnOnce(TryFromError) -> T) -> Result<Self, T> {
        TeacherRow::try_from(row).map(Into::into).map_err(err_map)
    }
}


impl Display for TeacherMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TeacherMetadata<{}> ({})", self.name.name_str(), self.id.id_str())
    }
}


#[graphql_object(
    context = Context,
    name = "Teacher",
    description = "This type represents the a specific teacher in the database.",
)]
impl TeacherMetadata {
    async fn absence_state(&self) -> AbsenceStateMetadata {
        AbsenceStateMetadata::from_id_and_meta(self.id.clone(), self.absence_state_meta)
    }

    fn id(&self) -> &TeacherId {
        &self.id
    }
    fn name(&self) -> &TeacherName {
        &self.name
    }

    fn pronouns(&self) -> &PronounSet {
        &self.pronouns
    }

    fn honorific(&self) -> &str {
        &self.honorific
    }
}

