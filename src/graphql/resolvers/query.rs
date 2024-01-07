//! This module contains solely the [`QueryRoot`] struct.
//! It only exists for organizational purposes.

// mod get_teacher;
// mod all_teachers;
// mod all_periods;


use crate::database::prepared::privileges::get_privileges;
use crate::state::AppState;

use crate::types::Privileges;
use crate::types::Teacher;
use crate::types::Period;
use crate::types::PackedAbsenceState;
use crate::types::TeacherAbsenceStateList;

use async_graphql::{
    Object,

    Context,
    Result as GraphQlResult,
    Error as GraphQlError,
};

use chrono::NaiveDate;
use uuid::Uuid;


/// This is a memberless struct implementing all the queries for `improved-eureka`.
/// This includes:
/// - `get_teacher(name?, id?) -> Teacher`
/// - `all_teachers() -> Teacher[]`
/// 
/// - `all_periods() -> Period[]`
/// - `get_period(name?, id?) -> Period`
/// 
/// Generally, it will only be used as part of a [schema][super::Schema].
#[derive(Debug, Clone, Copy)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn get_teacher(
        &self,
        ctx_accessor: &Context<'_>,
        #[graphql(desc = "Id of teacher")] id: Uuid,
    ) -> GraphQlResult<Teacher> {
        use crate::database::prepared::teacher::get_teacher as get_teacher_from_db;

        let ctx = ctx_accessor.data::<AppState>()?;

        let mut db_conn = ctx.db()
            .acquire()
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Could not open connection to the database {e}"))
            })?;

        get_teacher_from_db(&mut db_conn, id)
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Failed to get teacher from database {e}"))
            })
    }

    async fn all_teachers(
        &self,
        ctx: &Context<'_>,
    ) -> GraphQlResult<Vec<Teacher>> {
        use crate::database::prepared::teacher::get_all_teachers as get_all_teachers_from_db;        
        
        let ctx_accessor = ctx;
        let ctx = ctx_accessor.data::<AppState>()?;

        let mut db_conn = ctx.db()
            .acquire()
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Could not open connection to the database {e}"))
            })?;

            

        get_all_teachers_from_db(&mut db_conn)
            .await
            .map_err(|e| {
                if matches!(e, sqlx::Error::RowNotFound) {
                    GraphQlError::new_with_source("Teacher not found")
                } else {
                    let e = e.to_string();
                    GraphQlError::new(format!("Failed to get teacher from database {e}"))
                }
            })
    }

    async fn get_teacher_by_oauth(
        &self,
        ctx_accessor: &Context<'_>,
        #[graphql(desc = "Provider of OAuth")] provider: String,
        #[graphql(desc = "Sub of OAuth")] sub: String,
    ) -> GraphQlResult<Teacher> {
        use crate::database::prepared::teacher::get_teacher_by_oauth as get_teacher_by_oauth_from_db;

        let ctx = ctx_accessor.data::<AppState>()?;

        let mut db_conn = ctx.db()
            .acquire()
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Could not open connection to the database {e}"))
            })?;

        get_teacher_by_oauth_from_db(&mut db_conn, provider, sub)
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Failed to get teacher from database {e}"))
            })
    }

    async fn get_teacher_futures(
        &self,
        ctx_accessor: &Context<'_>,
        id: Uuid,
        start: NaiveDate,
        end: NaiveDate,
        #[graphql(desc = "Provider of OAuth")] provider: String,
        #[graphql(desc = "Sub of OAuth")] sub: String,
    ) -> GraphQlResult<Vec<PackedAbsenceState>> {
        use crate::database::prepared::future_absences::get_future_days_for_teacher as get_futures_from_db;
        use crate::database::prepared::teacher::check_teacher_oauth as check_oauth_db;


        let ctx = ctx_accessor.data::<AppState>()?;

        let mut db_conn = ctx.db()
            .acquire()
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Could not open connection to the database {e}"))
            })?;

        let oauth_res = check_oauth_db(&mut db_conn, id, provider, sub)
            .await
            .map_err(|_| GraphQlError::new("Not permitted to access this resource"))?;
        if !oauth_res {
            return Err(GraphQlError::new("Not permitted to access this resource"));
        }

        get_futures_from_db(&mut db_conn, id, start, end)
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Failed to get teacher absence data from database {e}"))
            })
    }
    async fn get_all_teacher_futures(
        &self,
        ctx_accessor: &Context<'_>,
        start: NaiveDate,
        end: NaiveDate,
        #[graphql(desc = "Provider of OAuth")] provider: String,
        #[graphql(desc = "Sub of OAuth")] sub: String,
    ) -> GraphQlResult<Vec<TeacherAbsenceStateList>> {
        use crate::database::prepared::future_absences::get_all_future_days as get_all_futures_from_db;
        use crate::database::prepared::teacher::get_teacher_by_oauth as get_teacher_db;


        let ctx = ctx_accessor.data::<AppState>()?;

        let mut db_conn = ctx.db()
            .acquire()
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Could not open connection to the database {e}"))
            })?;


        let teacher = get_teacher_db(&mut db_conn, provider.clone(), sub.clone())
            .await
            .map_err(|_| GraphQlError::new("This oauth user doesn't exist"))?;

        let teacher_perms = get_privileges(&mut db_conn, teacher.get_id())
            .await
            .map_err(|_| GraphQlError::new("Not permitted to access this resource"))?;

        if !teacher_perms.secretary && !teacher_perms.admin {
            return Err(GraphQlError::new("Not permitted to access this resource"));
        }


        get_all_futures_from_db(&mut db_conn, start, end)
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Failed to get teacher absence data from database {e}"))
            })
    }

    async fn all_periods(
        &self,
        ctx: &Context<'_>,
    ) -> GraphQlResult<Vec<Period>> {
        use crate::database::prepared::period::get_all_periods as get_all_periods_from_db;        
        
        let ctx_accessor = ctx;
        let ctx = ctx_accessor.data::<AppState>()?;

        let mut db_conn = ctx.db()
            .acquire()
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Could not open connection to the database {e}"))
            })?;

            

        get_all_periods_from_db(&mut db_conn)
            .await
            .map_err(|e| {
                if matches!(e, sqlx::Error::RowNotFound) {
                    GraphQlError::new_with_source("Period not found")
                } else {
                    let e = e.to_string();
                    GraphQlError::new(format!("Failed to get period from database {e}"))
                }
            })
    }




    async fn get_privs(
        &self,
        ctx_accessor: &Context<'_>,
        #[graphql(desc = "Provider of OAuth")] provider: String,
        #[graphql(desc = "Sub of OAuth")] sub: String,
    ) -> GraphQlResult<Privileges> {
        use crate::database::prepared::privileges::get_privileges as get_privs_from_db;
        use crate::database::prepared::teacher::get_teacher_by_oauth as get_teacher_db;

        let ctx = ctx_accessor.data::<AppState>()?;

        let mut db_conn = ctx.db()
            .acquire()
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Could not open connection to the database {e}"))
            })?;


        let teacher = get_teacher_db(&mut db_conn, provider.clone(), sub.clone())
            .await
            .map_err(|_| GraphQlError::new("This oauth user doesn't exist"))?;
        
        get_privs_from_db(&mut db_conn, teacher.get_id())
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Failed to get permissions for oauth user {e}"))
            })
    }

    async fn curr_spreadsheet_id(
        &self,
        ctx_accessor: &Context<'_>,
    ) -> GraphQlResult<String> {
        use crate::database::prepared::clients::get_sheet_id as get_sheet_id_from_db;

        let ctx = ctx_accessor.data::<AppState>()?;

        let mut db_conn = ctx.db()
            .acquire()
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Could not open connection to the database {e}"))
            })?;

        get_sheet_id_from_db(&mut db_conn)
            .await
            .map_err(|e| {
                let e = e.to_string();
                GraphQlError::new(format!("Failed to get teacher from database {e}"))
            })
    }
}




