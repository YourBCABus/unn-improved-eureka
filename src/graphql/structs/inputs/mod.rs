pub mod teacher_name;
pub mod teacher;

pub use {
    teacher_name::{ JuniperHonorific, JuniperMiddleName, JuniperTeacherName },
    teacher::JuniperTeacher,
};

use std::fmt::Debug;

use juniper::GraphQLInputObject;


#[derive(Debug, Clone, Copy, GraphQLInputObject)]
pub struct TimeRangeInput {
    pub start: f64,
    pub end: f64,
}

#[derive(juniper::GraphQLScalarValue, Debug, Clone)]
pub struct JuniperUuid(String);
impl JuniperUuid {
    pub fn new(id: &uuid::Uuid) -> Self {
        Self(id.to_string())
    }
    pub fn id_str(&self) -> &str {
        &self.0
    }
    pub fn uuid(&self) -> uuid::Uuid {
        self.try_uuid().unwrap_or_default()
    }
    pub fn try_uuid(&self) -> Result<uuid::Uuid, String> {
        uuid::Uuid::try_from(self.id_str()).map_err(|_| self.clone_to_string())
    }
    pub fn try_into_uuid(self) -> Result<(uuid::Uuid, Self), Self> {
        match uuid::Uuid::try_from(self.id_str()) {
            Ok(uuid) => Ok((uuid, self)),
            Err(_) => Err(self)
        }
    }
    pub fn into_string(self) -> String {
        self.0
    }
    pub fn clone_to_string(&self) -> String {
        self.0.clone()
    }
}
