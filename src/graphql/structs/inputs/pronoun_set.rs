use crate::types::PronounSet;

use async_graphql::InputObject;

#[derive(Debug, Clone, InputObject)]
pub struct GraphQlPronounSet {
    pub (super) sub: String,
    #[graphql(name = "obj")]
    pub (super) object: String,
    pub (super) pos_adj: String,
    pub (super) pos_pro: String,
    pub (super) refx: String,

    pub (super) gramm_plu: bool,
}
impl From<GraphQlPronounSet> for PronounSet {
    fn from(value: GraphQlPronounSet) -> Self {
        let GraphQlPronounSet { sub, object, pos_adj, pos_pro, refx, gramm_plu } = value;

        PronounSet::new(sub, object, pos_adj, pos_pro, refx, gramm_plu)
    }
}
