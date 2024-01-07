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
