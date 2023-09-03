use async_graphql::Object;

use crate::types::{Teacher, PronounSet, TeacherName};
use uuid::Uuid;

#[Object]
impl Teacher {
    // async fn absence_state(&self) -> AbsenceStateMetadata {
    //     AbsenceStateMetadata::from_id_and_meta(self.id.clone(), self.absence_state_meta)
    // }

    async fn id(&self) -> Uuid {
        self.get_id()
    }

    async fn pronouns(&self) -> &PronounSet {
        &self.get_pronouns()
    }

    async fn name(&self) -> &TeacherName {
        self.get_name()
    }
}

