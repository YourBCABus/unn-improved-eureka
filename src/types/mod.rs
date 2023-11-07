mod absence;
mod period;
mod teacher;

pub use teacher::{ Teacher, TeacherName, Honorific, pronouns::PronounSet };
pub use period::Period;
pub use absence::{ Absence, PackedAbsenceState, TeacherAbsenceStateList };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Privileges {
    pub secretary: bool,
    pub admin: bool,
}
