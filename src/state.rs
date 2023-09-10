use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct WebContext {
    db: PgPool,
}
impl WebContext {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[derive(Clone)]
pub struct AppState(std::sync::Arc<WebContext>);
impl AppState {
    pub fn new(db: PgPool) -> Self {
        Self(std::sync::Arc::new(WebContext::new(db)))
    }

    pub fn db(&self) -> &PgPool {
        &self.0.db
    }
}


