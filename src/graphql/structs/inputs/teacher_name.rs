use crate::types::TeacherName;


use async_graphql::{
    ScalarType,
    InputValueResult, InputValueError,
    Value, Scalar,
};

#[derive(Debug, Clone)]
pub struct GraphQlHonorific(crate::types::Honorific);

#[Scalar]
impl ScalarType for GraphQlHonorific {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = &value {
            if let Some(honorific) = crate::types::Honorific::try_from_str(&value) {
                Ok(GraphQlHonorific(honorific))
            } else {
                Err(InputValueError::custom(format!("Invalid honorific {value}")))
            }
        } else {
            // If the type does not match
            Err(InputValueError::expected_type(value))
        }
    }
    fn to_value(&self) -> Value {
        self.0.str().into()
    }
}


use async_graphql::InputObject;

#[derive(Debug, Clone, InputObject)]
pub struct GraphQlMiddleName {
    name: String,
    vis: bool,
}

#[derive(Debug, Clone, InputObject)]
pub struct GraphQlTeacherName {
    pub (super) honorific: GraphQlHonorific,
    pub (super) first: String,
    pub (super) last: String,
    pub (super) middle: Vec<GraphQlMiddleName>,
}
impl From<GraphQlTeacherName> for TeacherName {
    fn from(value: GraphQlTeacherName) -> Self {
        let GraphQlTeacherName { honorific, first, last, middle } = value;
        let middle = middle.into_iter().map(|GraphQlMiddleName { name, vis }| (vis, name)).collect();

        TeacherName::new(honorific.0, first, last, middle)
    }
}
