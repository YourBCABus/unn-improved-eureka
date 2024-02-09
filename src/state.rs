use sqlx::PgPool;

use crate::metrics::MetricProducer;

#[derive(Debug, Clone)]
pub struct WebContext {
    db: PgPool,
    metrics: MetricProducer,
}
impl WebContext {
    pub fn new(db: PgPool, metrics: MetricProducer) -> Self {
        Self { db, metrics }
    }
}

#[derive(Clone)]
pub struct AppState(std::sync::Arc<WebContext>);
impl AppState {
    pub fn new(db: PgPool, metrics: MetricProducer) -> Self {
        Self(std::sync::Arc::new(WebContext::new(db, metrics)))
    }

    pub fn db(&self) -> &PgPool {
        &self.0.db
    }

    pub fn metrics(&self) -> &MetricProducer {
        &self.0.metrics
    }
}


