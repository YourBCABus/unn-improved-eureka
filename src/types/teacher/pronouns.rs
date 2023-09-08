use serde::{ Serialize, Deserialize };

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct PronounSet {
    pub sub: String,
    pub object: String,
    pub pos_adj: String,
    pub pos_pro: String,
    pub refx: String,

    pub gramm_plu: bool,
}

impl PronounSet {
    pub fn new(
        sub: String, obj: String,
        pos_adj: String, pos_pro: String,
        refx: String, gramm_plu: bool,
    ) -> PronounSet {
        Self {
            sub, object: obj,
            pos_adj, pos_pro,
            refx, gramm_plu,
        }
    }
}


impl std::fmt::Debug for PronounSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"PronounSetInput<{}/{}/{}/{}/{}; "{} {}...">"#,
            self.sub,
            self.object,
            self.pos_adj,
            self.pos_pro,
            self.refx,

            self.sub,
            if self.gramm_plu { "are" } else { "is" },
        )
    }
}
