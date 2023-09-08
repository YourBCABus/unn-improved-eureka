//! This module contains solely the [QueryRoot] struct.
//! It only exists for organizational purposes.

// mod get_teacher;
// mod all_teachers;
// mod all_periods;


use crate::{state::AppState, types::Period};
use crate::types::Teacher;

use async_graphql::{
    Object,

    Context,
    Result as GraphQlResult,
    Error as GraphQlError,
};

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
                GraphQlError::new(format!("Failed to get teachers from database {e}"))
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
}




