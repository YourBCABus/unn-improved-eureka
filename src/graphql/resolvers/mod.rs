pub mod query;
pub mod mutation;

mod teacher;
mod period;

mod packed_absence_state;
mod time_range;
mod pronoun_set;
mod teacher_name;
mod privileges;

pub use {
    // teacher::TeacherMetadata,
    // absence_state::AbsenceStateMetadata,
    time_range::TimeRange,
};

macro_rules! get_db {
    ($ctx_accessor:expr) => {
        {
            let ctx = $ctx_accessor.data::<$crate::state::AppState>()?;
            ctx.db()
                .acquire()
                .await
                .map_err(|e| {
                    let e = e.to_string();
                    async_graphql::Error::new(format!("Could not open connection to the database {e}"))
                })?
        }
    };
}
pub (crate) use get_db;

macro_rules! run_query {
    (
        $db_conn:ident.$query_name:ident
        ($($var:expr),*$(,)?)
        else
            ($req_id:expr)
            $fmt_str:tt $(, $($fmt_args:expr),+ $(,)?)?
    ) => {
        $crate::graphql::resolvers::run_query!(
            $db_conn.($query_name)
            ($($var),*)

            else
                ($req_id)
                $fmt_str $(, $($fmt_args),+)?
        )
    };
    (
        $db_conn:ident.($query_name:expr)
        ($($var:expr),*$(,)?)
        else
            ($req_id:expr)
            $fmt_str:tt $(, $($fmt_args:expr),+ $(,)?)?
    ) => {
        ($query_name)(&mut $db_conn, $($var),*)
            .await
            .map_err(|e| {
                let e = e.to_string();
                $crate::logging::error!(
                    "{} - {}",
                    $crate::logs_env::logging::fmt_req_id($req_id),
                    format_args!($fmt_str, $($($fmt_args,)+)? e),
                );
                async_graphql::Error::new(format!($fmt_str, $($($fmt_args,)+)? e))
            })
    };
}
pub (crate) use run_query;


macro_rules! ensure_auth {
    ($ctx:ident, [$($scopes:ident),+]) => {
        {
            let scopes = $crate::graphql::get_scopes($ctx).await?;
            $(
                if !scopes.$scopes {
                    return Err(async_graphql::Error::new("Unauthorized"));
                }
            )+
        }
    };
}
pub (crate) use ensure_auth;
