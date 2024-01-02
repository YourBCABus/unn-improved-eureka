use sqlx::query_as;
use uuid::Uuid;

use super::super::Ctx;

#[derive(Debug, Clone)]
pub struct ClientKey { client_key: String }

pub async fn get_client_secret(ctx: &mut Ctx, id: Uuid) -> Result<Option<String>, sqlx::Error> {
    let get_key_query = query_as!(
        ClientKey,
        r#"
            SELECT client_key
            FROM clients
            WHERE id = $1;
        "#,
        id,
    );

    let res = get_key_query.fetch_optional(&mut **ctx).await?;

    Ok(res.map(|key| key.client_key))
}
