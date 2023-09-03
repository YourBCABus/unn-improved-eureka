use juniper::graphql_object;

use super::absence_state::AbsenceStateMetadata;

use crate::graphql::structs::inputs::JuniperUuid;
use crate::state::AppState;
use crate::types::{Teacher, PronounSet, TeacherName};



// /// This struct represents a Teacher with no period information associated with the absence_state.
// /// 
// /// This is mainly used to be an graphql-compatible type for [TeacherRow],
// /// and usually should be created with [From] or [Into].
// /// 
// /// [juniper] will apply resolvers to it to get the required fields.
// #[derive(Debug, Clone)]
// pub struct TeacherMetadata {
//     /// The id of the teacher. This is essentially a wrapper for a UUID, but is (de)serializable for juniper.
//     pub id: Uuid,

//     /// The parts of the name a teacher. See [NameParts] for more info.
//     pub name: NameParts,

//     /// The pronouns of the teacher. Contains:
//     /// - The entire set of `[sub, obj, posadj, pospro, ref]`.
//     /// - Grammatical plurality information. (Like `he is` vs `they are`)
//     /// (This model supports neopronouns.)
//     /// 
//     /// Examples:
//     /// - `[ she,  her,   her,   hers,  herself], false`
//     /// - `[  he,  him,   his,    his,  himself], false`
//     /// - `[they, them, their, theirs, themself], true`
//     /// - `[  xe,  xem,   xyr,   xyrs,  xemself], false`
//     pub pronouns: PronounSet,
    
//     /// The stripped absence state of the teacher.
//     pub absence_state_meta: TeacherPresence,
// }

// impl From<TeacherRow> for TeacherMetadata {
//     fn from(row: TeacherRow) -> Self {
//         Self {
//             id: row.id,
//             name: NameParts::new(row.first_name, row.last_name, row.honorific),
//             pronouns: row.pronoun_set,
//             absence_state_meta: row.presence,
//         }
//     }
// }

// type TryFromError = <TeacherRow as TryFrom<tokio_postgres::Row>>::Error;
// impl TeacherMetadata {
//     pub fn try_from_row<T>(row: tokio_postgres::Row, err_map: impl FnOnce(TryFromError) -> T) -> Result<Self, T> {
//         TeacherRow::try_from(row).map(Into::into).map_err(err_map)
//     }
// }


// impl Display for TeacherMetadata {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "TeacherMetadata<{}> ({})", self.name.full_name(), self.id.id_str())
//     }
// }


#[graphql_object(
    context = AppState,
    name = "Teacher",
    description = "This type represents the a specific teacher in the database.",
)]
impl Teacher {
    // async fn absence_state(&self) -> AbsenceStateMetadata {
    //     AbsenceStateMetadata::from_id_and_meta(self.id.clone(), self.absence_state_meta)
    // }

    fn id(&self) -> JuniperUuid {
        JuniperUuid::new(&self.id())
    }

    fn pronouns(&self) -> &PronounSet {
        &self.pronouns()
    }

    fn honorific(&self) -> &str {
        self.name().honorific()
    }

    fn name(&self) -> &TeacherName {
        self.name()
    }
}

