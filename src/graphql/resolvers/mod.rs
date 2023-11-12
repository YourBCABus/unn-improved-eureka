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
