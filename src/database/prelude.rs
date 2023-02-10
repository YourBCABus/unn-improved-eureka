//! This is just a convenience module that wraps the most important items in `improved-eureka`'s [database][crate::database] module.
//! Contained are glob imports of `database` and the [prepared] module.

pub use super::*;
pub use prepared::*;

pub use tokio_postgres::{Row, Statement};
pub use table_schemas::Teachers::TeacherPresence::TeacherPresence;
pub use crate::utils::structs::TeacherRow;

