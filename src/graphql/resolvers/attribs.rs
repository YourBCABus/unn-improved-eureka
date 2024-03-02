use std::collections::HashMap;
use sqlx::types::JsonValue;

use async_graphql::{
    Object,

    Context,
    Result as GraphQlResult,
    Error as GraphQlError,
};

use super::ensure_auth;

pub type AttribsInner = HashMap<String, JsonValue>;

pub struct Attribs(pub AttribsInner);

pub struct RawAttribs<'a>(&'a Attribs);

#[Object]
impl Attribs {
    async fn support_form_url<'a>(&'a self, ctx: &Context<'_>) -> GraphQlResult<Option<&'a str>> {
        ensure_auth!(ctx, [read_teacher, read_teacher_name, read_teacher_pronouns, read_teacher_absence, read_period]);
        match self.0.get("supportFormUrl") {
            Some(JsonValue::String(s)) => Ok(Some(s.as_str())),
            Some(_) => Err(GraphQlError::new("Support form URL is not set to a string")),
            _ => Ok(None),
        }
    }

    async fn raw<'a>(&'a self, ctx: &Context<'_>) -> GraphQlResult<RawAttribs<'a>> {
        ensure_auth!(ctx, [admin]);
        Ok(RawAttribs(self))
    }
}

#[Object]
impl<'a> RawAttribs<'a> {
    async fn get_key(&self, key: String) -> &JsonValue {
        match self.0.0.get(&key) {
            Some(v) => v,
            None => &JsonValue::Null,
        }
    }
    
    async fn get_all(&self) -> &AttribsInner {
        &self.0.0
    }
}
