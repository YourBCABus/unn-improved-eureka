//! This module contains solely the [MutationRoot] struct.
//! It only exists for organizational purposes.

// mod add_teacher;
// mod update_teacher;
// mod delete_teacher;

// mod add_period;
// mod update_period;
// mod delete_period;


// mod update_teacher_absence;
// mod clear_absences;
// mod clear_temp_times;

// use crate::graphql_types::{
//     scalars::teacher::*,
//     scalars::period::*,
//     juniper_types::IntoFieldError,
//     *, inputs::{TimeRangeInput, PronounSetInput},
// };

// use super::{teacher::TeacherMetadata, period::PeriodMetadata};

use async_graphql::{
    Object,

    Context,
    Result as GraphQlResult,
    Error as GraphQlError,
};
use chrono::NaiveDate;
use uuid::Uuid;

use crate::database::prepared::teacher::get_teacher;
use crate::types::Period;
use crate::{
    types::Teacher,
    state::AppState,
};

use crate::graphql::structs::{
    GraphQlTeacherName,
    GraphQlPronounSet, TimeRangeInput,
};


macro_rules! ensure_auth {
    ($ctx:ident, db: $db_conn:expr) => {
        {
            use $crate::verification::{ ClientIdHeader, ClientSecretHeader };
            use $crate::verification::id_secret::client_allowed;

            let client_id = $ctx.data::<ClientIdHeader>();
            let client_secret = $ctx.data::<ClientSecretHeader>();

            let (Ok(client_id), Ok(client_secret)) = (client_id, client_secret) else {
                return Err(GraphQlError::new("Unauthorized"));
            };

            if !client_allowed(client_id.unwrap(), client_secret.as_bytes(), $db_conn).await {
                return Err(GraphQlError::new("Unauthorized"));
            }
        }
    };
}

/// This is a memberless struct implementing all the mutations for `improved-eureka`.
/// This includes:
/// - `add_teacher(name?, id?) -> Teacher`
/// - `delete_teacher() -> bool`
/// 
/// Generally, it will only be used as part of a [schema][super::Schema].
#[derive(Debug, Clone, Copy)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn add_teacher(
        &self,
        ctx_accessor: &Context<'_>,
        name: GraphQlTeacherName,
        pronouns: GraphQlPronounSet,
    ) -> GraphQlResult<Teacher> {
        use crate::database::prepared::teacher::create_teacher as add_teacher_to_db;

        let ctx = ctx_accessor.data::<AppState>()?;

        let mut db_conn = ctx.db()
            .acquire()
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Could not open connection to the database {e}"))
            })?;

        ensure_auth!(ctx_accessor, db: &mut db_conn);

        let teacher = Teacher::new(
            uuid::Uuid::new_v4(),
            name.into(),
            pronouns.into(),
        );

        add_teacher_to_db(&mut db_conn, teacher)
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Database error: {e}"))
            })
    }
    
    async fn update_teacher_name(
        &self,
        ctx_accessor: &Context<'_>,
        id: Uuid,
        name: GraphQlTeacherName,
    ) -> GraphQlResult<Teacher> {
        use crate::database::prepared::teacher::update_teacher_name as update_teacher_name_in_db;

        let ctx = ctx_accessor.data::<AppState>()?;

        let mut db_conn = ctx.db()
            .acquire()
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Could not open connection to the database {e}"))
            })?;

        ensure_auth!(ctx_accessor, db: &mut db_conn);

        update_teacher_name_in_db(&mut db_conn, id, name.into())
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Database error: {e}"))
            })
    }

    async fn update_teacher_pronouns(
        &self,
        ctx_accessor: &Context<'_>,
        id: Uuid,
        pronouns: GraphQlPronounSet,
    ) -> GraphQlResult<Teacher> {
        use crate::database::prepared::teacher::update_teacher_pronouns as update_teacher_pronouns_in_db;

        let ctx = ctx_accessor.data::<AppState>()?;

        let mut db_conn = ctx.db()
            .acquire()
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Could not open connection to the database {e}"))
            })?;

        ensure_auth!(ctx_accessor, db: &mut db_conn);

        update_teacher_pronouns_in_db(&mut db_conn, id, pronouns.into())
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Database error: {e}"))
            })
    }
    
    async fn update_teacher_absence(
        &self,
        ctx_accessor: &Context<'_>,
        id: Uuid,
        periods: Vec<Uuid>,
        fully_absent: bool,
    ) -> GraphQlResult<Teacher> {
        use crate::database::prepared::absences::update_absences_for_teacher as update_absences_for_teacher_in_db;

        let ctx = ctx_accessor.data::<AppState>()?;

        let mut db_conn = ctx.db()
            .acquire()
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Could not open connection to the database {e}"))
            })?;

        ensure_auth!(ctx_accessor, db: &mut db_conn);

        update_absences_for_teacher_in_db(&mut db_conn, id, &periods, fully_absent)
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Database error: {e}"))
            })?;
        
        get_teacher(&mut db_conn, id)
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Database error: {e}"))
            })
    }

    async fn add_teacher_associated_oauth(
        &self,
        ctx_accessor: &Context<'_>,
        id: Uuid,
        provider: String,
        sub: String,
    ) -> GraphQlResult<Teacher> {
        use crate::database::prepared::teacher::add_teacher_oauth as add_teacher_associated_oauth_in_db;

        let ctx = ctx_accessor.data::<AppState>()?;

        let mut db_conn = ctx.db()
            .acquire()
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Could not open connection to the database: {e}"))
            })?;

        ensure_auth!(ctx_accessor, db: &mut db_conn);

        add_teacher_associated_oauth_in_db(&mut db_conn, id, provider, sub)
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Database error: {e}"))
            })?;

        get_teacher(&mut db_conn, id)
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Database error: {e}"))
            })
    }

    async fn remove_teacher_associated_oauth(
        &self,
        ctx_accessor: &Context<'_>,
        id: Uuid,
        provider: String,
    ) -> GraphQlResult<Teacher> {
        use crate::database::prepared::teacher::remove_teacher_oauth as remove_teacher_associated_oauth_in_db;

        let ctx = ctx_accessor.data::<AppState>()?;

        let mut db_conn = ctx.db()
            .acquire()
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Could not open connection to the database: {e}"))
            })?;

        ensure_auth!(ctx_accessor, db: &mut db_conn);

        remove_teacher_associated_oauth_in_db(&mut db_conn, id, provider)
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Database error: {e}"))
            })?;

        get_teacher(&mut db_conn, id)
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Database error: {e}"))
            })
    }


    #[allow(clippy::too_many_arguments)]
    async fn set_teacher_future_absence(
        &self,
        ctx_accessor: &Context<'_>,
        start: NaiveDate,
        end: Option<NaiveDate>,
        id: Uuid,
        periods: Vec<Uuid>,
        fully_absent: bool,
        comment: Option<String>,
    ) -> GraphQlResult<bool> {
        use crate::database::prepared::future_absences::set_future_day as set_future_absence_in_db;

        let ctx = ctx_accessor.data::<AppState>()?;

        let mut db_conn = ctx.db()
            .acquire()
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Could not open connection to the database {e}"))
            })?;

        ensure_auth!(ctx_accessor, db: &mut db_conn);

        set_future_absence_in_db(
            &mut db_conn,
            start, end.unwrap_or(start), id,
            &periods, fully_absent, comment,
        )
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Database error: {e}"))
            })?;
        
        Ok(true)
    }

    async fn clear_teacher_future_absence(
        &self,
        ctx_accessor: &Context<'_>,
        start: NaiveDate,
        end: Option<NaiveDate>,
        id: Uuid,
    ) -> GraphQlResult<bool> {
        use crate::database::prepared::future_absences::clear_future_day as clear_future_absence_in_db;

        let ctx = ctx_accessor.data::<AppState>()?;

        let mut db_conn = ctx.db()
            .acquire()
            .await
            .map_err(|e| {
                let e = e.to_string();

                GraphQlError::new(format!("Could not open connection to the database {e}"))
            })?;

        ensure_auth!(ctx_accessor, db: &mut db_conn);

        clear_future_absence_in_db(
            &mut db_conn,
            start, end.unwrap_or(start), id,
        )
            .await
            .map_err(|e| {
                let e = e.to_string();

                GraphQlError::new(format!("Database error: {e}"))
            })?;
        
        Ok(true)
    }

    async fn sync_and_flush_futures(
        &self,
        ctx_accessor: &Context<'_>,
    ) -> GraphQlResult<bool> {
        use crate::database::prepared::future_absences::flush_today as sync_and_flush_in_db;

        let ctx = ctx_accessor.data::<AppState>()?;

        let mut db_conn = ctx.db()
            .acquire()
            .await
            .map_err(|e| {
                let e = e.to_string();

                GraphQlError::new(format!("Could not open connection to the database {e}"))
            })?;

        ensure_auth!(ctx_accessor, db: &mut db_conn);

        sync_and_flush_in_db(&mut db_conn)
            .await
            .map_err(|e| {
                let e = e.to_string();

                GraphQlError::new(format!("Database error: {e}"))
            })?;
        
        Ok(true)
    }


    // async fn delete_teacher(
    //     ctx: &Context,
    //     id: TeacherId,
    // ) -> juniper::FieldResult<bool> {
    //     let mut db_context_mut = ctx.get_db_mut().await;

    //     delete_teacher
    //         ::delete_teacher(&mut db_context_mut.client, id)
    //         .await
    //         .map_err(IntoFieldError::into_field_error)?;
    //     Ok(true)
    // }


    async fn add_period(
        &self,
        ctx_accessor: &Context<'_>,

        name: String,
        default_time: TimeRangeInput,
    ) -> GraphQlResult<Period> {
        use crate::database::prepared::period::create_period as add_period_to_db;

        let ctx = ctx_accessor.data::<AppState>()?;

        let mut db_conn = ctx.db()
            .acquire()
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Could not open connection to the database {e}"))
            })?;

        ensure_auth!(ctx_accessor, db: &mut db_conn);

        add_period_to_db(&mut db_conn, &name, [default_time.start, default_time.end])
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Database error: {e}"))
            })
    }

    async fn update_period_name(
        &self,
        ctx_accessor: &Context<'_>,

        id: Uuid,
        name: String,
    ) -> GraphQlResult<Period> {
        use crate::database::prepared::period::update_period_name as update_period_name_in_db;

        let ctx = ctx_accessor.data::<AppState>()?;

        let mut db_conn = ctx.db()
            .acquire()
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Could not open connection to the database {e}"))
            })?;

        ensure_auth!(ctx_accessor, db: &mut db_conn);

        update_period_name_in_db(&mut db_conn, id, &name)
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Database error: {e}"))
            })
    }
    async fn update_period_time(
        &self,
        ctx_accessor: &Context<'_>,

        id: Uuid,
        time: TimeRangeInput,
    ) -> GraphQlResult<Period> {
        use crate::database::prepared::period::update_period_time as update_period_time_in_db;

        let ctx = ctx_accessor.data::<AppState>()?;

        let mut db_conn = ctx.db()
            .acquire()
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Could not open connection to the database {e}"))
            })?;

        ensure_auth!(ctx_accessor, db: &mut db_conn);

        update_period_time_in_db(&mut db_conn, id, [time.start, time.end])
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Database error: {e}"))
            })
    }
    async fn set_period_temp_time(
        &self,
        ctx_accessor: &Context<'_>,

        id: Uuid,
        temp_time: TimeRangeInput,
    ) -> GraphQlResult<Period> {
        use crate::database::prepared::period::set_period_temp_time as set_period_temp_time_in_db;

        let ctx = ctx_accessor.data::<AppState>()?;

        let mut db_conn = ctx.db()
            .acquire()
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Could not open connection to the database {e}"))
            })?;

        ensure_auth!(ctx_accessor, db: &mut db_conn);

        set_period_temp_time_in_db(&mut db_conn, id, [temp_time.start, temp_time.end])
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Database error: {e}"))
            })
    }

    // async fn delete_period(
    //     ctx: &Context,
    //     id: PeriodId,
    // ) -> juniper::FieldResult<bool> {
    //     let mut db_context_mut = ctx.get_db_mut().await;

    //     delete_period
    //         ::delete_period(&mut db_context_mut.client, id)
    //         .await
    //         .map_err(IntoFieldError::into_field_error)?;
    //     Ok(true)
    // }

    // async fn clear_absences(
    //     ctx: &Context,
    // ) -> juniper::FieldResult<bool> {
    //     let mut db_context_mut = ctx.get_db_mut().await;

    //     clear_absences
    //         ::clear_absences(&mut db_context_mut.client)
    //         .await
    //         .map_err(IntoFieldError::into_field_error)?;
    //     Ok(true)
    // }

    // async fn clear_temp_times(
    //     ctx: &Context,
    // ) -> juniper::FieldResult<bool> {
    //     let mut db_context_mut = ctx.get_db_mut().await;

    //     clear_temp_times
    //         ::clear_temp_times(&mut db_context_mut.client)
    //         .await
    //         .map_err(IntoFieldError::into_field_error)?;
    //     Ok(true)
    // }
}
