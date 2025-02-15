use std::fmt::{Debug, Display};

use async_graphql::{InputObject, InputValueError, InputValueResult, Scalar, ScalarType, Value};
use chrono::NaiveTime;
use uuid::Uuid;
use serde::{ Serialize, Deserialize };


#[derive(Clone, Serialize, Deserialize)]
pub struct Period {
    pub id: Uuid,

    pub name: String,
    pub short_name: Option<String>,

    pub start: SecondsSinceMidnight,
    pub end: SecondsSinceMidnight,
    
    
    pub temp_start: Option<f64>,
    pub temp_end: Option<f64>,

    pub is_temp: bool,
}

impl Debug for Period {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Period<{:?} ", self.name)?;
        if let Some(short) = self.short_name.as_ref() {
            write!(f, "({short:?}) ")?;
        }
        write!(f, "[from {} to {}] {}>", self.start.time(), self.end.time(), self.id.hyphenated())
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SecondsSinceMidnight(f64);

impl SecondsSinceMidnight {
    pub const SECONDS_PER_DAY: u64 = 24 * 60 * 60;

    pub fn seconds(&self) -> f64 {
        self.0
    }

    pub fn time(&self) -> NaiveTime {
        NaiveTime::from_num_seconds_from_midnight_opt(
            self.0 as u32,
            (self.0.fract() * 1_000_000_000.0) as u32,
        ).unwrap_or_default()
    }
}

impl From<f64> for SecondsSinceMidnight {
    fn from(value: f64) -> Self {
        if (0.0..Self::SECONDS_PER_DAY as f64).contains(&value) {
            Self(value)
        } else {
            Self(0.0)
        }
    }
}

#[Scalar]
impl ScalarType for SecondsSinceMidnight {
    fn parse(value: Value) -> InputValueResult<Self> {
        const MESSAGE: &str = "Invalid number of seconds since midnight";

        let async_graphql::Value::Number(value) = &value else {
            // If the type does not match
            return Err(InputValueError::expected_type(value));
        };

        let Some(value) = value.as_f64() else {
            return Err(InputValueError::custom(MESSAGE));
        };

        if (0.0..Self::SECONDS_PER_DAY as f64).contains(&value) {
            Ok(Self(value))
        } else {
            Err(InputValueError::custom(MESSAGE))
        } 
    }
    fn to_value(&self) -> Value {
        self.0.into()
    }
}

#[derive(Debug, Clone, Copy, InputObject)]
#[graphql(input_name = "TimeRangeInput")]
pub struct TimeRange {
    pub start: SecondsSinceMidnight,
    pub end: SecondsSinceMidnight,
}

impl TimeRange {
    pub fn new(start: SecondsSinceMidnight, end: SecondsSinceMidnight) -> Self {
        Self { start, end }
    }
}


