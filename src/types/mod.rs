mod absence;
mod period;
mod teacher;

pub use teacher::{ Teacher, TeacherName, MiddleName, Honorific, pronouns::PronounSet };
pub use period::{ Period, TimeRange, SecondsSinceMidnight };
pub use absence::{ Absence, PackedAbsenceState, TeacherAbsenceStateList };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Privileges {
    pub secretary: bool,
    pub admin: bool,
}
