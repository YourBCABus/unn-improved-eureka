use async_graphql::Context;
use uuid::Uuid;


use crate::graphql::resolvers::{get_db, run_query};
use crate::graphql::req_id;

use crate::graphql::structs::{GraphQlTeacherName, GraphQlPronounSet};
use crate::types::Teacher;

use async_graphql::Result as GraphQlResult;

pub async fn add_teacher(
    ctx: &Context<'_>,
    name: GraphQlTeacherName,
    pronouns: GraphQlPronounSet,
) -> GraphQlResult<Teacher> {
    use crate::database::prepared::teacher::create_teacher as add_teacher_to_db;

    let mut db_conn = get_db!(ctx);

    let teacher = Teacher::new(
        uuid::Uuid::new_v4(),
        name.into(),
        pronouns.into(),
    );
    let teacher_id = teacher.get_id();

    run_query!(
        db_conn.add_teacher_to_db(teacher)
        else (req_id(ctx)) "Failed to add teacher under ID {teacher_id}: {}"
    )
}


pub async fn update_teacher_name(
    ctx: &Context<'_>,
    id: Uuid,
    name: GraphQlTeacherName,
) -> GraphQlResult<Teacher> {
    use crate::database::prepared::teacher::update_teacher_name as update_teacher_name_in_db;

    let mut db_conn = get_db!(ctx);

    run_query!(
        db_conn.update_teacher_name_in_db(id, name.into())
        else (req_id(ctx)) "Failed to update name of teacher {id}: {}"
    )
}

pub async fn update_teacher_pronouns(
    ctx: &Context<'_>,
    id: Uuid,
    pronouns: GraphQlPronounSet,
) -> GraphQlResult<Teacher> {
    use crate::database::prepared::teacher::update_teacher_pronouns as update_teacher_pronouns_in_db;

    let mut db_conn = get_db!(ctx);

    run_query!(
        db_conn.update_teacher_pronouns_in_db(id, pronouns.into())
        else (req_id(ctx)) "Failed to update pronouns of teacher {id}: {}"
    )
}
