#![allow(unused_braces)]


use async_graphql::Object;
use crate::types::PronounSet;


#[Object]
impl PronounSet {
    async fn sub(&self) -> &str { &self.sub }
    async fn subject(&self) -> &str { &self.sub }

    async fn obj(&self) -> &str { &self.object }
    async fn object(&self) -> &str { &self.object }
 
    async fn pos_adj(&self) -> &str { &self.pos_adj }
    async fn poss_adjective(&self) -> &str { &self.pos_adj }

    async fn pos_pro(&self) -> &str { &self.pos_pro }
    async fn poss_pronoun(&self) -> &str { &self.pos_pro }

    async fn refx(&self) -> &str { &self.refx }
    async fn reflexive(&self) -> &str { &self.refx }

    async fn gramm_plu(&self) -> bool { self.gramm_plu }
    async fn grammatically_plural(&self) -> bool { self.gramm_plu }

    #[graphql(complexity = 2)]
    async fn set_str(&self) -> String {
        format!("{};{};{};{};{};{}", self.sub, self.object, self.pos_adj, self.pos_pro, self.refx, self.gramm_plu)
    }
}
