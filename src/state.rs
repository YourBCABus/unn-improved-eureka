use sqlx::PgPool;

#[derive(Debug)]
pub struct WebContext {
    pub db: PgPool,
    pub schema: crate::graphql::Schema,
}


pub type AppState = std::sync::Arc<WebContext>;

