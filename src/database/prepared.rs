//! This crate contains [modifying] and [read] for the two flavors of SQL query.
//! See their documentation for more information.

pub mod teacher;
pub mod period;
pub mod absences;

pub mod future_absences;
pub mod privileges;

pub mod clients;

macro_rules! prepared_query {
    (
        $query_text:literal;
        $($($lifetimes:lifetime),+)? { $($name:ident: $type:ty),* $(,)? };
        $($vars:expr),* $(,)?
    ) => {
        {
            #[allow(dead_code)]
            struct QueryResult$(<$($lifetimes),+>)? {
                $($name: $type),*
            }
    
            sqlx::query_as!(
                QueryResult,
                $query_text,
                $($vars),*
            )
        }
    };
}
pub (crate) use prepared_query;
