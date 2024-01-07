#[allow(clippy::module_name_repetitions)]
pub mod teacher_name;

#[allow(clippy::module_name_repetitions)]
pub mod teacher;

#[allow(clippy::module_name_repetitions)]
pub mod pronoun_set;


pub use {
    teacher_name::{ GraphQlHonorific, GraphQlMiddleName, GraphQlTeacherName },
    teacher::GraphQlTeacher,
    pronoun_set::GraphQlPronounSet,
};

use std::fmt::Debug;

use async_graphql::InputObject;

#[derive(Debug, Clone, Copy, InputObject)]
pub struct TimeRangeInput {
    pub start: f64,
    pub end: f64,
}
