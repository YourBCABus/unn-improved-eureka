pub use super::*;
pub use prepared::*;

use std::sync::Arc;
pub fn easy_build_db_context(client: Client) -> Arc<DbContext> {
    Arc::new(DbContext { client })
}
