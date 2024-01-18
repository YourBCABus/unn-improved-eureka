use sqlx::query_as;
use uuid::Uuid;

use super::super::Ctx;
use super::prepared_query;
use crate::types::Period;



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

                EXTRACT(EPOCH FROM temp_start)::float as temp_start,
                EXTRACT(EPOCH FROM temp_end)::float as temp_end
            FROM periods
            WHERE id = $1;
        "#,
        id,
    );

    get_period_query.fetch_one(&mut **ctx).await
}

pub async fn get_all_periods(ctx: &mut Ctx) -> Result<Vec<Period>, sqlx::Error> {
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
                EXTRACT(EPOCH FROM temp_end)::float as temp_end
            FROM periods;
        "#,
    );

    get_all_periods_query.fetch_all(&mut **ctx).await
}

pub async fn create_period(ctx: &mut Ctx, name: &str, time_range: [f64; 2]) -> Result<Period, sqlx::Error> {
    let add_period = prepared_query!(
        r#"
            INSERT INTO periods (id, name, start_time, end_time)
            VALUES (
                uuid_generate_v4(), $1,
                TIME '00:00' + $2 * INTERVAL '1 second',
                TIME '00:00' + $3 * INTERVAL '1 second'
            ) RETURNING id AS "id: _";
        "#;
        { id: Uuid };
        name,
        time_range[0], time_range[1],
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

pub async fn update_period_time(ctx: &mut Ctx, id: Uuid, time_range: [f64; 2]) -> sqlx::Result<Period> {
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
        time_range[0], time_range[1],
    );
    
    update_time.execute(&mut **ctx).await?;
    get_period(ctx, id).await
}

pub async fn set_period_temp_time(ctx: &mut Ctx, id: Uuid, temp_time_range: [f64; 2]) -> sqlx::Result<Period> {
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
        temp_time_range[0], temp_time_range[1],
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

