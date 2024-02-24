use uuid::Uuid;

use crate::verification::scopes::Scopes;

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

pub async fn get_client_scopes(ctx: &mut Ctx, id: Uuid) -> Result<Option<Scopes>, sqlx::Error> {
    let get_scopes_query = prepared_query!(
        r"
            SELECT scopes
            FROM clients
            WHERE id = $1;
        ";
        { scopes: String };
        id
    );

    let res = get_scopes_query.fetch_optional(&mut **ctx).await?;

    Ok(res.and_then(|scopes| Scopes::try_from_str(&scopes.scopes)))
}
