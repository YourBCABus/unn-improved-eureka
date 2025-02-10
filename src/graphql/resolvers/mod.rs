pub mod query;
pub mod mutation;

mod teacher;
mod period;

mod packed_absence_state;
mod time_range;
mod pronoun_set;
mod teacher_name;
mod privileges;
mod sparse_metrics_view;
mod attribs;

pub use {
    // teacher::TeacherMetadata,
    // absence_state::AbsenceStateMetadata,
    time_range::TimeRange,
};

macro_rules! get_db {
    (<state> $ctx:expr) => {
        {
            match $ctx.db().acquire().await {
                Ok(conn) => conn,
                Err(e) => {
                    let pool = $ctx.db();
                    let (idle, total) = (pool.num_idle(), pool.size());
                    let options = format!("{:?}", pool.options());

                    crate::logging::error!("DB Error: {e:?}");
                    crate::report!("Database error when getting scopes": {
                        "db_info": {
                            "conns": {
                                "idle": idle,
                                "total": total,
                            },
                            "opts": options,
                        },
                    });

                    let e = e.to_string();
                    return Err(async_graphql::Error::new(format!("Could not open connection to the database {e}")))
                }
            }
        }
    };
    ($ctx_accessor:expr) => {
        {
            let ctx = $ctx_accessor.data::<$crate::state::AppState>()?;
            $crate::graphql::resolvers::get_db!(<state> ctx)
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
        match ($query_name)(&mut $db_conn, $($var),*).await {
            Ok(res) => Ok(res),
            Err(e) => {
                let e = e.to_string();
                $crate::logging::error!(
                    "{} - {}",
                    $crate::logs_env::logging::fmt_req_id($req_id),
                    format_args!($fmt_str, $($($fmt_args,)+)? e),
                );
                $crate::report!("Failed to run query": {
                    "query_name": stringify!($query_name),
                    "error": e.to_string(),
                });
                Err(async_graphql::Error::new(format!($fmt_str, $($($fmt_args,)+)? e)))
            }
        }
    };
}
pub (crate) use run_query;


macro_rules! ensure_auth {
    ($ctx:ident, [$($scopes:ident),+]) => {
        {
            // $crate::logging::trace!("Getting scopes...");
            let (id_secret_scopes, id_token_scopes) = tokio::try_join! {
                $crate::graphql::get_scopes_id_secret($ctx),
                $crate::graphql::get_scopes_id_token($ctx),
            }?;

            let scopes = id_secret_scopes | id_token_scopes;
            $(
                if !scopes.$scopes && !scopes.admin {
                    return Err(async_graphql::Error::new("Unauthorized"));
                }
            )+
        }
    };
}
pub (crate) use ensure_auth;

// macro_rules! get_db {
//     () => {
        
//     };
// }
