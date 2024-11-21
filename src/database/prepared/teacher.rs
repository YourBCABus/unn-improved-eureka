use constant_time_eq::constant_time_eq;
use sqlx::{ query, query_as, Connection };
use uuid::Uuid;

use super::super::Ctx;
use super::prepared_query;
use crate::types::{Teacher, TeacherName, PronounSet, Honorific};


#[derive(Debug, Clone, sqlx::FromRow)]
pub struct SqlTeacherInfo {
    id: Uuid,
    fully_absent: bool,

    #[allow(unused)]
    pro_id: Uuid,
    pro_sub: String,
    pro_obj: String,
    pro_pos_adj: String,
    pro_pos_pro: String,
    pro_refx: String,
    pro_gramm_plu: bool,

    #[allow(unused)]
    name_name_of: Uuid,
    name_honorific: String,
    name_first: String,
    name_last: String,
    name_middle_texts: Vec<String>,
    name_middle_display: Vec<bool>,
}
impl From<SqlTeacherInfo> for Option<Teacher> {
    fn from(sql: SqlTeacherInfo) -> Self {
        let SqlTeacherInfo {
            id,
            fully_absent,

            pro_id: _,
            pro_sub, pro_obj,
            pro_pos_adj, pro_pos_pro,
            pro_refx, pro_gramm_plu,

            name_name_of: _,
            name_honorific,
            name_first, name_last,
            name_middle_texts, name_middle_display,
        } = sql;
        

        let middles = name_middle_display.into_iter().zip(name_middle_texts).collect();
        let honorific = Honorific::try_from_str(&name_honorific)?;

        let name = TeacherName::new(honorific, name_first, name_last, middles);

        let pronouns = PronounSet {
            sub: pro_sub, object: pro_obj,
            pos_adj: pro_pos_adj, pos_pro: pro_pos_pro,
            refx: pro_refx, gramm_plu: pro_gramm_plu,
        };

        Some(Teacher::new(id, name, pronouns).with_fully_absence(fully_absent))
    }
}

pub async fn get_teacher(ctx: &mut Ctx, id: Uuid) -> Result<Teacher, sqlx::Error> {
    let get_teacher_query = query_as!(
        SqlTeacherInfo,
        r#"
            SELECT
                t.id, t.fully_absent,

                p.id AS pro_id,
                p.sub AS pro_sub, p.obj AS pro_obj,
                p.pos_adj AS pro_pos_adj, p.pos_pro AS pro_pos_pro,
                p.refx AS pro_refx, p.gramm_plu AS pro_gramm_plu,

                n.name_of AS name_name_of,
                n.honorific AS name_honorific,
                n.first AS name_first, n.last AS name_last,
                n.middle_texts AS name_middle_texts, n.middle_display AS name_middle_display
            FROM teachers AS t
                INNER JOIN pronoun_sets AS p ON t.pronouns = p.id
                INNER JOIN names AS n ON t.id = n.name_of
                WHERE t.id = $1;
        "#,
        id,
    );

    let teacher_info = get_teacher_query.fetch_one(&mut **ctx).await?;

    Option::from(teacher_info)
        .ok_or_else(|| sqlx::Error::ColumnNotFound(id.to_string()))
}

pub async fn get_all_teachers(ctx: &mut Ctx) -> Result<Vec<Teacher>, sqlx::Error> {
    let get_all_teachers_query = query_as!(
        SqlTeacherInfo,
        r#"
            SELECT
                t.id, t.fully_absent,

                p.id AS pro_id,
                p.sub AS pro_sub, p.obj AS pro_obj,
                p.pos_adj AS pro_pos_adj, p.pos_pro AS pro_pos_pro,
                p.refx AS pro_refx, p.gramm_plu AS pro_gramm_plu,

                n.name_of AS name_name_of,
                n.honorific AS name_honorific,
                n.first AS name_first, n.last AS name_last,
                n.middle_texts AS name_middle_texts, n.middle_display AS name_middle_display
            FROM teachers AS t
                INNER JOIN pronoun_sets AS p ON t.pronouns = p.id
                INNER JOIN names AS n ON t.id = n.name_of;
        "#,
    );

    let teacher_info = get_all_teachers_query.fetch_all(&mut **ctx).await?;

    Ok(teacher_info.into_iter().filter_map(Option::from).collect())
}




pub async fn create_teacher(ctx: &mut Ctx, input: Teacher) -> Result<Teacher, sqlx::Error> {
    let PronounSet {
        sub, object: obj,
        pos_adj, pos_pro,
        refx, gramm_plu,
    } = input.get_pronouns();

    let id = input.get_id();

    let add_pronoun_set = prepared_query!(
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
        "#;
        { id: Uuid };
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

    let add_teacher = query(r"
        INSERT INTO teachers (id, pronouns)
        VALUES ($1, $2);
    ");


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

pub async fn update_teacher_pronouns(ctx: &mut Ctx, id: Uuid, pronouns: PronounSet) -> sqlx::Result<Teacher> {

    let PronounSet {
        sub, object: obj,
        pos_adj, pos_pro,
        refx, gramm_plu,
    } = pronouns;


    let add_pronoun_set = prepared_query!(
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
        "#;
        { id: Uuid };
        sub, obj,
        pos_adj, pos_pro,
        refx, gramm_plu,
    );

    let update_teacher_id = query(
        r"
            UPDATE teachers 
            SET pronouns = $2
            WHERE id = $1;
        "
    );


    ctx.transaction(|txn| Box::pin(async move {
        let pronoun_set_id = add_pronoun_set.fetch_one(&mut **txn).await?.id;

        update_teacher_id.bind(id).bind(pronoun_set_id).execute(&mut **txn).await

    })).await?;

    get_teacher(ctx, id).await
}


pub async fn update_teacher_full_absence(ctx: &mut Ctx, id: Uuid, fully_absent: bool) -> sqlx::Result<Teacher> {
    let update_absence = query!(
        r#"
            UPDATE teachers
            SET fully_absent = $2
            WHERE id = $1;
        "#,
        id,
        fully_absent
    );

    update_absence.execute(&mut **ctx).await?;

    get_teacher(ctx, id).await
}



pub async fn get_teacher_by_oauth(ctx: &mut Ctx, provider: String, sub: String) -> Result<Teacher, sqlx::Error> {
    let teacher_oauth_query = prepared_query!(
        r#"
            SELECT teacher as id
            FROM teacher_oauths
            WHERE
                provider = $1 AND
                sub = $2;
        "#;
        { id: Uuid };
        provider,
        sub,
    );

    let teacher_id = teacher_oauth_query.fetch_one(&mut **ctx).await?.id;

    get_teacher(ctx, teacher_id).await
}
pub async fn check_teacher_oauth(ctx: &mut Ctx, id: Uuid, provider: String, sub: String) -> Result<bool, sqlx::Error> {
    let get_sub_query = query!(
        r#"
            SELECT sub
            FROM teacher_oauths
            WHERE
                provider = $1 AND
                teacher = $2;
        "#,
        provider,
        id,
    );

    let db_sub = get_sub_query.fetch_one(&mut **ctx).await?.sub;
    
    if db_sub.len() != sub.len() {
        return Ok(false);
    }

    Ok(constant_time_eq(db_sub.as_bytes(), sub.as_bytes()))
}

pub async fn add_teacher_oauth(ctx: &mut Ctx, teacher: Uuid, provider: String, sub: String) -> Result<(), sqlx::Error> {
    let add_teacher_oauth = query!(
        r#"
            INSERT INTO teacher_oauths (teacher, provider, sub)
            VALUES ($1, $2, $3);
        "#,
        teacher,
        provider,
        sub,
    );

    add_teacher_oauth.execute(&mut **ctx).await?;

    Ok(())
}

pub async fn remove_teacher_oauth(ctx: &mut Ctx, teacher: Uuid, provider: String) -> Result<(), sqlx::Error> {
    let remove_teacher_oauth = query!(
        r#"
            DELETE FROM teacher_oauths
            WHERE
                teacher = $1 AND
                provider = $2;
        "#,
        teacher,
        provider,
    );

    remove_teacher_oauth.execute(&mut **ctx).await?;

    Ok(())
}
