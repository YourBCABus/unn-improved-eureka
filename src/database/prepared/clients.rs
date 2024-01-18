use uuid::Uuid;

use super::super::Ctx;
use super::prepared_query;

pub async fn get_client_secret(ctx: &mut Ctx, id: Uuid) -> Result<Option<String>, sqlx::Error> {
    let get_key_query = prepared_query!(
        r"
            SELECT client_key
            FROM clients
            WHERE id = $1;
        ";
        { client_key: String };
        id
    );

    let res = get_key_query.fetch_optional(&mut **ctx).await?;

    Ok(res.map(|key| key.client_key))
}
