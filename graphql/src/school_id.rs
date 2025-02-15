use db::get_db;
use tokio::sync::OnceCell;

use uuid::Uuid;

pub struct SchoolId(OnceCell<Uuid>);

impl std::fmt::Debug for SchoolId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(id) = self.inner() {
            write!(f, "SchoolId({id})")
        } else {
            write!(f, "SchoolId(<empty>)")
        }
    }
}

impl SchoolId {
    pub fn new(id: Option<Uuid>) -> Self {
        let out = Self(OnceCell::new());
        if let Some(id) = id {
            out.set(id);
        }
        out
    }

    fn inner(&self) -> Option<Uuid> {
        self.0.get().copied()
    }

    fn set(&self, id: Uuid) {
        let _ = self.0.set(id);
    }
}

pub async fn get_school_id(context: &async_graphql::Context<'_>) -> async_graphql::Result<Uuid> {
    let Ok(scopes_cell) = context.data::<SchoolId>() else {
        let path_node = context.path_node.map(|node| node.to_string()).unwrap_or_default();

        logging::error!("SchoolId missing from context @ path node {path_node}!");
        logging::report!("SchoolId missing from context": { "path_node": path_node });

        return Err(async_graphql::Error::new("Server context error"));
    };

    scopes_cell.0.get_or_try_init(|| async {
        let mut db_conn = get_db!(context);
        get_default_school_id(&mut db_conn).await
    }).await.copied()
}

pub async fn get_default_school_id(ctx: &mut db::Ctx) -> async_graphql::Result<Uuid> {
    let get_default_school_id_query = db::prepared_query!(
        r"
            SELECT id
            FROM schools
            WHERE is_default = true;
        ";
        { id: Uuid };
    );

    match get_default_school_id_query.fetch_one(&mut **ctx).await {
        Ok(v) => Ok(v.id),
        Err(e) => {
            logging::error!("Failed to get default school id: {e}");
            logging::report!("Failed to get default school id": { "db_err": format!("{e:?}") });
            Err(async_graphql::Error::new("DB Error"))
        }
    }
}
