use std::collections::HashMap;

use sqlx::{ query, query_as, Connection };
use uuid::Uuid;

use super::super::Ctx;
use crate::types::{Teacher, TeacherName, PronounSet, Honorific};


#[derive(Debug, Clone, sqlx::FromRow)]
pub struct SqlTeacherName {
    name_of: Uuid,
    honorific: String,
    first: String,
    last: String,
    middle_texts: Vec<String>,
    middle_display: Vec<bool>,
}
impl From<SqlTeacherName> for Option<TeacherName> {
    fn from(sql: SqlTeacherName) -> Self {
        let SqlTeacherName { honorific, first, last, middle_texts, middle_display, .. } = sql;
        
        let middle = middle_display.into_iter().zip(middle_texts.into_iter()).collect();
        let honorific = Honorific::try_from_str(&honorific);

        honorific.map(|honorific| TeacherName::new(honorific, first, last, middle) )
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct SqlPronouns {
    id: Uuid,
    sub: String,
    obj: String,
    pos_adj: String,
    pos_pro: String,
    refx: String,
    gramm_plu: bool,
}
impl From<SqlPronouns> for PronounSet {
    fn from(value: SqlPronouns) -> Self {
        let SqlPronouns { id: _, sub, obj, pos_adj, pos_pro, refx, gramm_plu } = value;
        PronounSet { sub, object: obj, pos_adj, pos_pro, refx, gramm_plu }
    }
}

pub async fn get_teacher(ctx: &mut Ctx, id: Uuid) -> Result<Teacher, sqlx::Error> {
    let teacher_name_query = query_as!(
        SqlTeacherName,
        r#"
            SELECT
                name_of,
                first, last,
                middle_texts, middle_display,
                honorific
            FROM names
                WHERE name_of = $1;
        "#,
        id,
    );
    let teacher_pronouns_query = query_as!(
        SqlPronouns,
        r#"
            SELECT
                id,
                sub, obj,
                pos_adj, pos_pro,
                refx, gramm_plu
            FROM pronoun_sets
                WHERE id = (SELECT pronouns FROM teachers WHERE id = $1);
        "#,
        id,
    );

    let mut transaction = ctx.begin().await?;

    let name = teacher_name_query.fetch_one(&mut *transaction).await?;
    let pronouns = teacher_pronouns_query.fetch_one(&mut *transaction).await?;

    transaction.commit().await?;

    let pronouns = pronouns.into();

    let Some(name) = name.into() else {
        return Err(sqlx::Error::ColumnNotFound(id.to_string()));
    };

    Ok(Teacher::new(id, name, pronouns))
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct SqlTeacherJoiner {
    id: Uuid,
    pronouns: Uuid,
}

pub async fn get_all_teachers(ctx: &mut Ctx) -> Result<Vec<Teacher>, sqlx::Error> {
    let teacher_name_query = query_as!(
        SqlTeacherName,
        r#"
            SELECT
                name_of,
                first, last,
                middle_texts, middle_display,
                honorific
            FROM names;
        "#,
    );
    let teacher_pronouns_query = query_as!(
        SqlPronouns,
        r#"
            SELECT
                id,
                sub, obj,
                pos_adj, pos_pro,
                refx, gramm_plu
            FROM pronoun_sets;
        "#,
    );
    let teacher_joiner_query = query_as!(
        SqlTeacherJoiner,
        r#"
            SELECT id, pronouns FROM teachers;
        "#,
    );


    let mut transaction = ctx.begin().await?;

    let names = teacher_name_query.fetch_all(&mut *transaction).await?;
    let pronoun_sets = teacher_pronouns_query.fetch_all(&mut *transaction).await?;
    let joiners = teacher_joiner_query.fetch_all(&mut *transaction).await?;

    transaction.commit().await?;



    let mut names_map: HashMap<_, _> = names
        .into_iter()
        .map(|name| (name.name_of, name))
        .collect();

    let mut pronouns_map: HashMap<_, _> = pronoun_sets
        .into_iter()
        .map(|pronoun_set| (pronoun_set.id, pronoun_set))
        .collect();



    let teachers: Vec<_> = joiners
        .into_iter()
        .filter_map(|SqlTeacherJoiner { id, pronouns }| {
            let name = names_map.remove(&id);
            let pronouns = pronouns_map.remove(&pronouns);
            Some(Teacher::new(id, Option::from(name?)?, pronouns?.into()))
        })
        .collect();

    Ok(teachers)
}


// pub async fn update_chall(ctx: &mut Ctx, id: Uuid, input: ChallInput) -> Result<Option<Chall>, sqlx::Error> {
//     let query = query!(
//         r#"
//             UPDATE challenges
//             SET
//                 name = COALESCE($2, name),
//                 description = COALESCE($3, description),
//                 points = COALESCE($4, points),
//                 authors = COALESCE($5, authors),
//                 hints = COALESCE($6, hints),
//                 categories = COALESCE($7, categories),
//                 tags = COALESCE($8, tags),
//                 visible = COALESCE($9, visible),
//                 source_folder = COALESCE($10, source_folder)
//             WHERE id = $1;
//         "#,
//         id,
//         input.name: String,
//         input.description,
//         input.points,
//         input.authors.as_deref(),
//         input.hints.as_deref(),
//         input.categories.as_deref(),
//         input.tags.as_deref(),
//         input.visible,
//         input.source_folder,
//     );
//     let affected = query
//         .execute(&mut *ctx)
//         .await?
//         .rows_affected();

//     if affected != 1 { return Ok(None) }

//     if let Some(links) = input.links {
//         set_chall_links(&mut *ctx, id, links).await?;
//     }
//     set_chall_updated(&mut *ctx, id).await?;

//     let Some(output) = get_chall(ctx, id).await? else {
//         return Err(sqlx::Error::RowNotFound);
//     };

//     Ok(Some(output))
// }



struct Id { id: Uuid }

pub async fn create_teacher(ctx: &mut Ctx, input: Teacher) -> Result<Teacher, sqlx::Error> {
    let PronounSet {
        sub, object: obj,
        pos_adj, pos_pro,
        refx, gramm_plu,
    } = input.get_pronouns();

    let id = input.get_id();

    let add_pronoun_set = query_as!(
        Id,
        r#"
            INSERT INTO pronoun_sets (
                id,
                sub, obj,
                pos_adj, pos_pro,
                refx, gramm_plu
            )
            VALUES (uuid_generate_v4(), $1, $2, $3, $4, $5, $6)
                ON CONFLICT
                    ON CONSTRAINT nopronounsetduplicates
                    DO UPDATE SET id = pronoun_sets.id
                RETURNING id AS "id: _";
        "#,
        sub, obj,
        pos_adj, pos_pro,
        refx, gramm_plu,
    );
    

    let first = input.get_name().get_first();
    let last = input.get_name().get_last();
    let honorific = input.get_name().get_honorific().str();

    let middle_texts: Vec<_> = input.get_name().all_middles().map(|(_, name)| name.to_string()).collect();
    let middle_display: Vec<_> = input.get_name().all_middles().map(|(display, _)| display).collect();

    let add_name = query!(
        r#"
            INSERT INTO names (
                name_of,
                first, last,
                middle_texts, middle_display,
                honorific
            ) VALUES ($1, $2, $3, $4, $5, $6);
        "#,
        input.get_id(),
        first, last,
        middle_texts.as_slice(), &middle_display,
        honorific,
    );

    let add_teacher = query(r#"
        INSERT INTO teachers (id, pronouns)
        VALUES ($1, $2);
    "#);


    ctx.transaction(|txn| Box::pin(async move {
        
        let pronoun_set_id = add_pronoun_set.fetch_one(&mut **txn).await?.id;

        add_teacher.bind(id).bind(pronoun_set_id).execute(&mut **txn).await?;
        
        add_name.execute(&mut **txn).await
    })).await?;

    get_teacher(ctx, id).await
}

pub async fn update_teacher_name(ctx: &mut Ctx, id: Uuid, name: TeacherName) -> sqlx::Result<Teacher> {
    let first = name.get_first();
    let last = name.get_last();
    let honorific = name.get_honorific().str();

    let middle_texts: Vec<_> = name.all_middles().map(|(_, name)| name.to_string()).collect();
    let middle_display: Vec<_> = name.all_middles().map(|(display, _)| display).collect();

    let add_name = query!(
        r#"
            UPDATE names
            SET
                first = $2,
                last = $3,
                middle_texts = $4,
                middle_display = $5,
                honorific = $6
            WHERE
                name_of = $1;
        "#,
        id,
        first, last,
        middle_texts.as_slice(), &middle_display,
        honorific,
    );

    add_name.execute(&mut **ctx).await?;

    get_teacher(ctx, id).await
}

