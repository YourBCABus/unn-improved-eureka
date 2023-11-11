use crate::types::Teacher;
use super::{teacher_name::GraphQlTeacherName, pronoun_set::GraphQlPronounSet};

use async_graphql::InputObject;
use uuid::Uuid;

#[derive(Debug, Clone, InputObject)]
pub struct GraphQlTeacher {
    pub (super) id: Option<Uuid>,
    pub (super) name: GraphQlTeacherName,
    pub (super) pronouns: GraphQlPronounSet,
}
impl From<GraphQlTeacher> for Teacher {
    fn from(value: GraphQlTeacher) -> Self {
        let GraphQlTeacher { id, name, pronouns } = value;

        let id = id.unwrap_or_else(uuid::Uuid::new_v4);
        let name = name.into();
        let pronouns = pronouns.into();
        
        Teacher::new(id, name, pronouns)
    }
}
