use std::fmt::Display;

use juniper::GraphQLInputObject;

use crate::graphql_types::Context;
use chrono::{NaiveTime, Timelike};



#[derive(Debug, Clone, Copy, GraphQLInputObject)]
pub struct TimeRangeInput {
    pub start: f64,
    pub end: f64,
}
