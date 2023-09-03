#![allow(unused_braces)]

use crate::types::PronounSet;
use crate::state::AppState;

use juniper::graphql_object;


#[graphql_object(
    context = AppState,
    name = "PronounSet",
)]
impl PronounSet {
    fn sub(&self) -> &str { &self.sub }
    fn subject(&self) -> &str { &self.sub }
    
    fn obj(&self) -> &str { &self.obj }
    fn object(&self) -> &str { &self.obj }
    
    fn pos_adj(&self) -> &str { &self.pos_adj }
    fn poss_adjective(&self) -> &str { &self.pos_adj }
    
    fn pos_pro(&self) -> &str { &self.pos_pro }
    fn poss_pronoun(&self) -> &str { &self.pos_pro }

    fn refx(&self) -> &str { &self.refx }
    fn reflexive(&self) -> &str { &self.refx }

    fn gramm_plu(&self) -> bool { self.gramm_plu }
    fn grammatically_plural(&self) -> bool { self.gramm_plu }

    fn set_str(&self) -> String {
        format!("{};{};{};{};{};{}", self.sub, self.obj, self.pos_adj, self.pos_pro, self.refx, self.gramm_plu)
    }
}
