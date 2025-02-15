use sqlx::{query, query_as};
use uuid::Uuid;

use db::Ctx;
use db::prepared_query;
use crate::types::Period;
use crate::types::TimeRange;



pub async fn get_period(ctx: &mut Ctx, id: Uuid) -> Result<Period, sqlx::Error> {
    let get_period_query = query_as!(
        Period,
        r#"
            SELECT
                id,
                name,
                null as short_name,

                EXTRACT(EPOCH FROM start_time)::float as "start!",
                EXTRACT(EPOCH FROM end_time)::float as "end!",

                EXTRACT(EPOCH FROM temp_start)::float as "temp_start!",
                EXTRACT(EPOCH FROM temp_end)::float as "temp_end!",

                is_temp
            FROM periods
            WHERE id = $1;
        "#,
        id,
    );

    get_period_query.fetch_one(&mut **ctx).await
}

pub async fn get_all_periods(ctx: &mut Ctx, school_id: Uuid) -> Result<Vec<Period>, sqlx::Error> {
    let get_all_periods_query = query_as!(
        Period,
        r#"
            SELECT
                id,
                name,
                null as short_name,

                EXTRACT(EPOCH FROM start_time)::float as "start!",
                EXTRACT(EPOCH FROM end_time)::float as "end!",

                EXTRACT(EPOCH FROM temp_start)::float as temp_start,
                EXTRACT(EPOCH FROM temp_end)::float as temp_end,

                is_temp
            FROM periods
            WHERE school_id = $1;
        "#,
        school_id,
    );

    get_all_periods_query.fetch_all(&mut **ctx).await
}

pub async fn create_period(ctx: &mut Ctx, school_id: Uuid, name: &str, time_range: TimeRange, temp: bool) -> Result<Period, sqlx::Error> {
    let add_period = prepared_query!(
        r#"
            INSERT INTO periods (id, name, start_time, end_time, is_temp, school_id)
            VALUES (
                uuid_generate_v4(), $1,
                TIME '00:00' + $2 * INTERVAL '1 second',
                TIME '00:00' + $3 * INTERVAL '1 second',
                $4,
                $5
            ) RETURNING id AS "id: _";
        "#;
        { id: Uuid };
        name,
        time_range.start.seconds(), time_range.end.seconds(),
        temp,
        school_id,
    );
    
    let id = add_period.fetch_one(&mut **ctx).await?.id;

    get_period(ctx, id).await
}

pub async fn update_period_name(ctx: &mut Ctx, id: Uuid, name: &str) -> sqlx::Result<Period> {
    let update_name = prepared_query!(
        r"
            UPDATE periods
            SET name = $2
            WHERE id = $1;
        ";
        {  };
        id,
        name
    );

    update_name.execute(&mut **ctx).await?;
    get_period(ctx, id).await
}    

pub async fn update_period_time(ctx: &mut Ctx, id: Uuid, time_range: TimeRange) -> sqlx::Result<Period> {
    let update_time = prepared_query!(
        r"
            UPDATE periods
            SET
                start_time = TIME '00:00' + $2 * INTERVAL '1 second',
                end_time = TIME '00:00' + $3 * INTERVAL '1 second'
            WHERE id = $1;
        ";
        {  };
        id,
        time_range.start.seconds(), time_range.end.seconds(),
    );
    
    update_time.execute(&mut **ctx).await?;
    get_period(ctx, id).await
}

pub async fn set_period_temp_time(ctx: &mut Ctx, id: Uuid, temp_time_range: TimeRange) -> sqlx::Result<Period> {
    let update_temp_time = prepared_query!(
        r"
            UPDATE periods
            SET
                temp_start = TIME '00:00' + $2 * INTERVAL '1 second',
                temp_end = TIME '00:00' + $3 * INTERVAL '1 second'
            WHERE id = $1;
        ";
        {  };
        id,
        temp_time_range.start.seconds(), temp_time_range.end.seconds(),
    );
    
    update_temp_time.execute(&mut **ctx).await?;
    get_period(ctx, id).await
}

pub async fn clear_period_temp_time(ctx: &mut Ctx, id: Uuid) -> sqlx::Result<Period> {
    let update_temp_time = prepared_query!(
        r"
            UPDATE periods
            SET temp_start = null, temp_end = null
            WHERE id = $1;
        ";
        {  };
        id,
    );
    
    update_temp_time.execute(&mut **ctx).await?;
    get_period(ctx, id).await
}

pub async fn flush_all_temp_times(ctx: &mut Ctx) -> sqlx::Result<()> {
    let flush_temp_times = prepared_query!(
        r"
            UPDATE periods
            SET temp_start = null, temp_end = null;
        ";
        {  };
    );
    
    flush_temp_times.execute(&mut **ctx).await?;
    Ok(())
}

pub async fn flush_all_temp_periods(ctx: &mut Ctx) -> sqlx::Result<()> {
    let flush_temp_times = prepared_query!(
        r"
            DELETE FROM periods
            WHERE is_temp;
        ";
        {  };
    );
    
    flush_temp_times.execute(&mut **ctx).await?;
    Ok(())
}

pub async fn delete_period(ctx: &mut Ctx, id: Uuid) -> Result<u64, sqlx::Error> {
    let delete_period_query = query!(
        r#"
            DELETE FROM periods
            WHERE id = $1;
        "#,
        id,
    );

    delete_period_query
        .execute(&mut **ctx)
        .await
        .map(|res| res.rows_affected())
}
