use std::collections::HashMap;
use std::sync::Arc;

use chrono::{NaiveDate, Days};
use sqlx::{ query, query_as };
use uuid::Uuid;

use crate::types::{PackedAbsenceState, Period, TeacherAbsenceStateList};

use super::super::Ctx;
use super::absences::update_absences_for_teacher;
use super::period::get_all_periods;


pub async fn set_future_day(
    ctx: &mut Ctx,

    start: NaiveDate,
    end: NaiveDate,
    id: Uuid,

    periods: &[Uuid],
    fully_absent: bool,
    comment: Option<String>,
) -> Result<(), sqlx::Error> {
    let start_days_since_epoch = start.signed_duration_since(NaiveDate::default()).num_days();
    let end_days_since_epoch = end.signed_duration_since(NaiveDate::default()).num_days();

    let add_or_update_future_date = query!(
        r#"
            INSERT INTO teacher_future_schedules (
                teacher, date,

                periods, fully_absent, comment
            )
            (
                SELECT
                    $1,
                    DATE '1/1/1970' + date_idx * INTERVAL '1 day',
                    $2,
                    $3,
                    $4
                FROM generate_series($5::bigint, $6::bigint) as date_idx
            )
                ON CONFLICT (teacher, date)
                DO UPDATE SET
                    periods = $2,
                    fully_absent = $3,
                    comment = $4;
        "#,
        id,
        &periods, fully_absent, comment,
        start_days_since_epoch, end_days_since_epoch,
    );

    add_or_update_future_date.execute(&mut **ctx).await?;

    Ok(())
}

pub async fn clear_future_day(
    ctx: &mut Ctx,
    
    start: NaiveDate,
    end: NaiveDate,
    id: Uuid,
) -> Result<(), sqlx::Error> {
    let start_days_since_epoch = start.signed_duration_since(NaiveDate::default()).num_days() as f64;
    let end_days_since_epoch = end.signed_duration_since(NaiveDate::default()).num_days() as f64;

    let remove_teacher_oauth = query!(
        r#"
            DELETE FROM teacher_future_schedules
            WHERE
                teacher = $1 AND
                daterange(
                    (DATE '1/1/1970' + $2 * INTERVAL '1 day')::date,
                    (DATE '1/1/1970' + $3 * INTERVAL '1 day')::date,
                    '[]'::text
                ) @> date;
        "#,
        id,
        start_days_since_epoch,
        end_days_since_epoch,
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

struct BarebonesFutureDay {
    pub teacher_id: Uuid,
    pub date: f64,
    pub periods: Vec<Uuid>,
    pub fully_absent: bool,
    pub comment: Option<String>,
}

#[derive(Debug, Clone)]
struct ColumnDecodeError {
    column: &'static str,
    got: String,
    expected_type: &'static str,
}

impl std::fmt::Display for ColumnDecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{column} could not be decoded into a {expected_type} from {source}",
            column = self.column,
            source = self.got,
            expected_type = self.expected_type,
        )
    }
}
impl std::error::Error for ColumnDecodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

fn get_packed_absence_state(future_day: BarebonesFutureDay, period_map: &HashMap<Uuid, Arc<Period>>) -> Result<PackedAbsenceState, sqlx::Error> {
    let BarebonesFutureDay { teacher_id, periods, date, fully_absent, comment } = future_day;

    let periods: Result<Vec<_>, _> = periods.into_iter()
        .map(|period_id| {
            let Some(period) = period_map.get(&period_id) else {
                return Err(sqlx::Error::RowNotFound);
            };
            Ok(period.clone())
        })
        .collect();

    let periods = periods?;

    

    let date = NaiveDate::default()
        .checked_add_days(Days::new(future_day.date as u64))
        .ok_or(sqlx::Error::ColumnDecode {
            index: "Date".to_string(),
            source: Box::new(ColumnDecodeError { column: "date", got: date.to_string(), expected_type: "Date" }),
        })?;

    Ok(PackedAbsenceState {
        teacher_id,
        date,
        periods,
        fully: fully_absent,
        comments: comment,
    })
}

pub async fn get_future_days_for_teacher(ctx: &mut Ctx, id: Uuid, start: NaiveDate, end: NaiveDate) -> Result<Vec<PackedAbsenceState>, sqlx::Error> {
    let start = start.signed_duration_since(NaiveDate::default()).num_days() as f64;
    let end = end.signed_duration_since(NaiveDate::default()).num_days() as f64;

    let get_future_days_in_range = query_as!(
        BarebonesFutureDay,
        r#"
            SELECT
                tfs.teacher as teacher_id,
                EXTRACT(EPOCH FROM date)::float / 86400 as "date!",
                periods,
                fully_absent,
                comment
            FROM teacher_future_schedules as tfs
            WHERE
                tfs.teacher = $1 AND
                DATE '1/1/1970' + $2 * INTERVAL '1 day' <= tfs.date AND
                tfs.date <= DATE '1/1/1970' + $3 * INTERVAL '1 day';
        "#,
        id,
        start,
        end,
    );

    let period_map: HashMap<_, _> = get_all_periods(ctx)
        .await?
        .into_iter()
        .map(|period| (period.id, Arc::new(period)))
        .collect();

    let data = get_future_days_in_range.fetch_all(&mut **ctx).await?;

    data.into_iter()
        .map(|future_day| get_packed_absence_state(future_day, &period_map))
        .collect()
}

pub async fn get_all_future_days(ctx: &mut Ctx, start: NaiveDate, end: NaiveDate) -> Result<Vec<TeacherAbsenceStateList>, sqlx::Error> {
    let start = start.signed_duration_since(NaiveDate::default()).num_days() as f64;
    let end = end.signed_duration_since(NaiveDate::default()).num_days() as f64;

    let get_all_future_days_in_range = query_as!(
        BarebonesFutureDay,
        r#"
            SELECT
                tfs.teacher as teacher_id,
                EXTRACT(EPOCH FROM date)::float / 86400 as "date!",
                periods,
                fully_absent,
                comment
            FROM teacher_future_schedules as tfs
            WHERE
                DATE '1/1/1970' + $1 * INTERVAL '1 day' <= tfs.date AND
                tfs.date <= DATE '1/1/1970' + $2 * INTERVAL '1 day'
            ORDER BY tfs.teacher;
        "#,
        start,
        end,
    );

    let period_map: HashMap<_, _> = get_all_periods(ctx)
        .await?
        .into_iter()
        .map(|period| (period.id, Arc::new(period)))
        .collect();

    let data = get_all_future_days_in_range.fetch_all(&mut **ctx).await?;

    let future_day_iterator = data.into_iter().map(|future_day| get_packed_absence_state(future_day, &period_map));
    

    let mut teacher_map = HashMap::new();

    for state in future_day_iterator {
        let state = state?;
        
        teacher_map.entry(state.teacher_id)
            .or_insert_with(Vec::new)
            .push(state);
    }

    Ok(teacher_map.into_iter().map(|(id, states)| TeacherAbsenceStateList(id, states)).collect())
}
