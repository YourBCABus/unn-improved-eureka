use async_graphql::Context;
use graphql::{get_school_id, req_id};
use uuid::Uuid;

use db::{ get_db, run_query };

use crate::types::{PronounSet, Teacher, TeacherName};

use async_graphql::Result as GraphQlResult;

pub async fn add_teacher(
    ctx: &Context<'_>,
    name: TeacherName,
    pronouns: PronounSet,
) -> GraphQlResult<Teacher> {
    use crate::queries::teacher::create_teacher as add_teacher_to_db;

    let mut db_conn = get_db!(ctx);
    let school_id = get_school_id(ctx).await?;
    logging::info!("Adding teacher to school {school_id:?}");

    let teacher = Teacher::new(
        uuid::Uuid::new_v4(),
        name,
        pronouns,
        None,
    );
    let teacher_id = teacher.get_id();

    run_query!(
        db_conn.add_teacher_to_db(school_id, teacher)
        else (req_id(ctx)) "Failed to add teacher under ID {teacher_id}: {}"
    )
}


pub async fn update_teacher_name(
    ctx: &Context<'_>,
    id: Uuid,
    name: TeacherName,
) -> GraphQlResult<Teacher> {
    use crate::queries::teacher::update_teacher_name as update_teacher_name_in_db;

    let mut db_conn = get_db!(ctx);

    run_query!(
        db_conn.update_teacher_name_in_db(id, name)
        else (req_id(ctx)) "Failed to update name of teacher {id}: {}"
    )
}

pub async fn update_teacher_pronouns(
    ctx: &Context<'_>,
    id: Uuid,
    pronouns: PronounSet,
) -> GraphQlResult<Teacher> {
    use crate::queries::teacher::update_teacher_pronouns as update_teacher_pronouns_in_db;

    let mut db_conn = get_db!(ctx);

    run_query!(
        db_conn.update_teacher_pronouns_in_db(id, pronouns)
        else (req_id(ctx)) "Failed to update pronouns of teacher {id}: {}"
    )
}

pub async fn update_teacher_comments(
    ctx: &Context<'_>,
    id: Uuid,
    comments: Option<String>,
) -> GraphQlResult<Teacher> {
    use crate::queries::teacher::update_teacher_comments as update_teacher_comments_in_db;

    let mut db_conn = get_db!(ctx);

    run_query!(
        db_conn.update_teacher_comments_in_db(id, comments)
        else (req_id(ctx)) "Failed to update comments of teacher {id}: {}"
    )
}
