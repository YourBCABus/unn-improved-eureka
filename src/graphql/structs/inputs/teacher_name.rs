use crate::types::TeacherName;


#[derive(Debug, Clone, juniper::GraphQLScalarValue)]
pub struct JuniperHonorific(crate::types::Honorific);

#[derive(Debug, Clone, juniper::GraphQLInputObject)]
pub struct JuniperMiddleName {
    name: String,
    vis: bool,
}

#[derive(Debug, Clone, juniper::GraphQLInputObject)]
pub struct JuniperTeacherName {
    pub (super) honorific: JuniperHonorific,
    pub (super) first: String,
    pub (super) last: String,
    pub (super) middle: Vec<JuniperMiddleName>,
}
impl From<JuniperTeacherName> for TeacherName {
    fn from(value: JuniperTeacherName) -> Self {
        let JuniperTeacherName { honorific, first, last, middle } = value;
        let middle = middle.into_iter().map(|JuniperMiddleName { name, vis }| (vis, name)).collect();
        TeacherName { honorific: honorific.0, first, last, middle }
    }
}
