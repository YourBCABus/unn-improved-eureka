use std::fmt::Debug;

use juniper::GraphQLInputObject;

#[derive(Debug, Clone, Copy, GraphQLInputObject)]
pub struct TimeRangeInput {
    pub start: f64,
    pub end: f64,
}

#[derive(Clone, GraphQLInputObject)]
pub struct PronounSetInput {
    pub subject: String,
    pub object: String,
    pub poss_adjective: String,
    pub poss_pronoun: String,
    pub reflexive: String,

    pub grammatically_plural: bool,
}

impl PronounSetInput {
    pub fn format_sql(&self) -> String {
        format!(
            "{};{};{};{};{};{}",
            self.subject, self.object,
            self.poss_adjective, self.poss_pronoun,
            self.reflexive,
            self.grammatically_plural,
        )
    }
}

impl Debug for PronounSetInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"PronounSetInput<{}/{}/{}/{}/{}; "{} {}...">"#,
            self.subject,
            self.object,
            self.poss_adjective,
            self.poss_pronoun,
            self.reflexive,

            self.subject,
            if self.grammatically_plural { "are" } else { "is" },
        )
    }
}
