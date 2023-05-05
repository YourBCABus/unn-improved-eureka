
// use std::{fmt::Display, ops::Range};

use std::fmt::Debug;


use crate::graphql_types::inputs::PronounSetInput;



#[derive(Clone, PartialEq, Eq)]
pub struct PronounSet {
    sub: String,
    obj: String,
    pos_adj: String,
    pos_pro: String,
    refx: String,

    gramm_plu: bool,
}

pub enum PronounSetPart {
    Subject, Object, PosAdj, PosPro, Refx,
    GrammPlu,
}

impl PronounSet {
    pub fn try_new(db_string: &str) -> Result<Self, PronounSetPart> {
        use PronounSetPart::*;

        let mut iterator = db_string.split(';').map(str::trim);

        let sub = iterator.next();
        let obj = iterator.next();
        let pos_adj = iterator.next();
        let pos_pro = iterator.next();
        let refx = iterator.next();

        let gramm_plu = iterator.next();

        let Some(sub) = sub else { return Err(Subject) };
        let Some(obj) = obj else { return Err(Object) };
        let Some(pos_adj) = pos_adj else { return Err(PosAdj) };
        let Some(pos_pro) = pos_pro else { return Err(PosPro) };
        let Some(refx) = refx else { return Err(Refx) };

        let Some(gramm_plu) = gramm_plu else { return Err(GrammPlu) };
        let gramm_plu = match gramm_plu {
            "true" => true,
            "false" => false,
            _ => return Err(GrammPlu),
        };

        let (sub, obj, pos_adj, pos_pro, refx) = (
            sub.to_string(),
            obj.to_string(),
            pos_adj.to_string(),
            pos_pro.to_string(),
            refx.to_string(),
        );

        Ok(Self { sub, obj, pos_adj, pos_pro, refx, gramm_plu, })
    }

    pub fn format_sql(&self) -> String {
        format!("{};{};{};{};{};{}", self.sub, self.obj, self.pos_adj, self.pos_pro, self.refx, self.gramm_plu)
    }
}

impl From<PronounSetInput> for PronounSet {
    fn from(value: PronounSetInput) -> Self {
        let PronounSetInput { subject: sub, object: obj, poss_adjective: pos_adj, poss_pronoun: pos_pro, reflexive: refx, grammatically_plural: gramm_plu } = value;
        Self {
            sub,
            obj,
            pos_adj,
            pos_pro,
            refx,

            gramm_plu,
        }
    }
}

impl Debug for PronounSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"PronounSetInput<{}/{}/{}/{}/{}; "{} {}...">"#,
            self.sub,
            self.obj,
            self.pos_adj,
            self.pos_pro,
            self.refx,

            self.sub,
            if self.gramm_plu { "are" } else { "is" },
        )
    }
}


mod braces_module {
    //! It's almost disappointing that this has to exist.
    #![allow(unused_braces)]
    use super::PronounSet;
    
    use juniper::graphql_object;
    use crate::graphql_types::Context;

    #[graphql_object(
        context = Context,
        name = "PronounSet",
    )]
    impl PronounSet {
        fn subject(&self) -> &str { &self.sub }
        
        fn object(&self) -> &str { &self.obj }
        
        fn poss_adjective(&self) -> &str { &self.pos_adj }
        
        fn poss_pronoun(&self) -> &str { &self.pos_pro }
    
        fn reflexive(&self) -> &str { &self.refx }

        fn grammatically_plural(&self) -> bool { self.gramm_plu }
    }
}


