use chrono::NaiveDate;
use sqlx::{ query, query_as };
use uuid::Uuid;

use super::super::Ctx;
use super::absences::update_absences_for_teacher;


pub async fn set_future_day(
    ctx: &mut Ctx,

    date: NaiveDate,
    id: Uuid,

    periods: &[Uuid],
    fully_absent: bool,
    comment: Option<String>,
) -> Result<(), sqlx::Error> {
    let days_since_epoch = date.signed_duration_since(NaiveDate::default()).num_days() as f64;

    let add_or_update_future_date = query!(
        r#"
            INSERT INTO teacher_future_schedules (
                teacher, date,

                periods, fully_absent, comment
            )
            VALUES ($1, DATE '1/1/1970' + $2 * INTERVAL '1 day', $3, $4, $5)
                ON CONFLICT (teacher, date)
                DO UPDATE SET
                    periods = $3,
                    fully_absent = $4,
                    comment = $5;
        "#,
        id, days_since_epoch,
        &periods, fully_absent, comment,
    );

    add_or_update_future_date.execute(&mut **ctx).await?;

    Ok(())
}

pub async fn clear_future_day(
    ctx: &mut Ctx,
    
    date: NaiveDate,
    id: Uuid,
) -> Result<(), sqlx::Error> {
    let days_since_epoch = date.signed_duration_since(NaiveDate::default()).num_days() as f64;

    let remove_teacher_oauth = query!(
        r#"
            DELETE FROM teacher_future_schedules
            WHERE
                teacher = $1 AND
                date = DATE '1/1/1970' + $2 * INTERVAL '1 day';
        "#,
        id,
        days_since_epoch,
    );

    remove_teacher_oauth.execute(&mut **ctx).await?;

    Ok(())
}



pub struct FutureDay {
    pub id: Uuid,
    pub date: f64,
    pub periods: Vec<Uuid>,
    pub fully_absent: bool,
    pub comment: Option<String>,
}

pub async fn flush_today(ctx: &mut Ctx) -> Result<(), sqlx::Error> {
    let get_futures_for_today = query_as!(
        FutureDay,
        r#"
            SELECT
                teacher as id,
                EXTRACT(EPOCH FROM date)::float / 86400 as "date!",
                periods,
                fully_absent,
                comment
            FROM teacher_future_schedules
            WHERE date = CURRENT_DATE;
        "#,
    );

    let remove_past = query!(
        r#"
            DELETE FROM teacher_future_schedules as tfs
            WHERE tfs.date <= CURRENT_DATE;
        "#,
    );

    let today_data = get_futures_for_today.fetch_all(&mut **ctx).await?;
    remove_past.execute(&mut **ctx).await?;

    for teacher_today in today_data {
        update_absences_for_teacher(ctx, teacher_today.id, &teacher_today.periods, teacher_today.fully_absent).await?;
    }
    Ok(())
}
