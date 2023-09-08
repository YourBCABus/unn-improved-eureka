use sqlx::query_as;
use uuid::Uuid;

use super::super::Ctx;
use crate::types::Absence;


pub async fn get_absence(ctx: &mut Ctx, id: Uuid) -> Result<Absence, sqlx::Error> {
    let get_absence_query = query_as!(
        Absence,
        r#"
            SELECT
                period_id as period,
                teacher_id as teacher
            FROM absence_xref
            WHERE id = $1;
        "#,
        id,
    );

    get_absence_query.fetch_one(&mut **ctx).await
}

pub async fn get_all_absences(ctx: &mut Ctx) -> Result<Vec<Absence>, sqlx::Error> {
    let get_all_absences_query = query_as!(
        Absence,
        r#"
            SELECT period_id as period, teacher_id as teacher FROM absence_xref;
        "#,
    );

    get_all_absences_query.fetch_all(&mut **ctx).await
}

pub async fn get_all_absences_for_period(ctx: &mut Ctx, id: Uuid) -> Result<Vec<Absence>, sqlx::Error> {
    let get_period_absences_query = query_as!(
        Absence,
        r#"
            SELECT
                period_id as period,
                teacher_id as teacher
            FROM absence_xref
            WHERE
                period_id = $1;
        "#,
        id
    );

    get_period_absences_query.fetch_all(&mut **ctx).await
}

pub async fn get_all_absences_for_teacher(ctx: &mut Ctx, id: Uuid) -> Result<Vec<Absence>, sqlx::Error> {
    let get_teacher_absences_query = query_as!(
        Absence,
        r#"
            SELECT
                period_id as period,
                teacher_id as teacher
            FROM absence_xref
            WHERE
                teacher_id = $1;
        "#,
        id
    );

    get_teacher_absences_query.fetch_all(&mut **ctx).await
}

struct Id { id: Uuid }
pub async fn add_absence(ctx: &mut Ctx, period: Uuid, teacher: Uuid) -> Result<Absence, sqlx::Error> {
    let add_absence_query = query_as!(
        Id,
        r#"
            INSERT INTO absence_xref (id, period_id, teacher_id)
            VALUES (uuid_generate_v4(), $1, $2)
            RETURNING id AS "id: _";
        "#,
        period,
        teacher
    );

    let id = add_absence_query.fetch_one(&mut **ctx).await?.id;

    get_absence(ctx, id).await
}

pub async fn remove_absence(ctx: &mut Ctx, period: Uuid, teacher: Uuid) -> Result<(), sqlx::Error> {
    let remove_absence_query = query_as!(
        Id,
        r#"
            DELETE FROM absence_xref
            WHERE
                period_id = $1 AND
                teacher_id = $2;
        "#,
        period,
        teacher
    );

    remove_absence_query.execute(&mut **ctx).await.map(|_| ())
}
pub async fn remove_absences_for_teacher(ctx: &mut Ctx, teacher: Uuid) -> Result<(), sqlx::Error> {
    let remove_absences_query = query_as!(
        Id,
        r#"
            DELETE FROM absence_xref
            WHERE
                teacher_id = $1;
        "#,
        teacher
    );

    remove_absences_query.execute(&mut **ctx).await.map(|_| ())
}

pub async fn update_absences_for_teacher(ctx: &mut Ctx, teacher: Uuid, periods: &[Uuid]) -> Result<Vec<Absence>, sqlx::Error> {

    remove_absences_for_teacher(ctx, teacher).await?;

    let mut absences = vec![];
    for period in periods {
        absences.push(add_absence(ctx, *period, teacher).await?);
    }
    Ok(absences)
}
