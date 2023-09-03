use crate::types::{Teacher, PronounSet};

use super::teacher_name::JuniperTeacherName;

#[derive(Debug, Clone, juniper::GraphQLInputObject)]
pub struct JuniperTeacher {
    pub (super) id: Option<String>,
    pub (super) name: JuniperTeacherName,
    pub (super) pronouns: PronounSet,
}
impl From<JuniperTeacher> for Teacher {
    fn from(value: JuniperTeacher) -> Self {
        let JuniperTeacher { id, name, pronouns } = value;

        Teacher { id: id.unwrap_or_else(|| uuid::Uuid::new_v4()), name: name.into(), pronouns }
    }
}
