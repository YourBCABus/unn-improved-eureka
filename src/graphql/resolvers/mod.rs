pub mod query;
pub mod mutation;

mod teacher;
mod absence_state;
mod period;
mod time_range;
mod pronoun_set;

pub use {
    teacher::TeacherMetadata,
    absence_state::AbsenceStateMetadata,
    period::PeriodMetadata,
    time_range::TimeRange,
    pronoun_set::PronounSet,
};
