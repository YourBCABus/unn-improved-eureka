use sqlx::PgPool;

pub struct WebContext {
    pub db: PgPool,
}

#[derive(Clone)]
pub struct AppState(std::sync::Arc<WebContext>);
impl AppState {
    pub fn new(db: PgPool) -> Self {
        Self(std::sync::Arc::new(WebContext { db }))
    }

    pub fn db(&self) -> &PgPool {
        &self.0.db
    }
}


