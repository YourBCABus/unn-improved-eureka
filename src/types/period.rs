use std::fmt::{Debug, Display};

use uuid::Uuid;
use serde::{ Serialize, Deserialize };


#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Period {
    pub id: Uuid,

    pub name: String,
    pub  short_name: Option<String>,

    pub start: f64,
    pub end: f64,
    
    pub temp_start: Option<f64>,
    pub temp_end: Option<f64>,
}

impl Debug for Period {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Period<{:?} ", self.name)?;
        if let Some(short) = self.short_name.as_ref() {
            write!(f, "({:?}) ", short)?;
        }
        write!(f, "[from {} to {}] {}", self.start, self.end, self.id.hyphenated())
    }
}
impl Display for Period {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let (true, Some(short)) = (f.alternate(), self.short_name.as_ref()) {
            write!(f, "{short}")
        } else {
            write!(f, "{}", self.name)
        }
    }
}
