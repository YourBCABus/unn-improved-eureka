use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct Absence {
    pub teacher: Uuid,
    pub period: Uuid,
}