use uuid::Uuid;

use crate::verification::scopes::Scopes;

use super::super::Ctx;
use super::prepared_query;

pub async fn get_client_secret(ctx: &mut Ctx, client_id: Uuid, school_id: Uuid) -> Result<Option<String>, sqlx::Error> {
    let get_key_query = prepared_query!(
        r"
            SELECT client_key
            FROM clients
            WHERE id = $1 AND school_id = $2;
        ";
        { client_key: String };
        client_id,
        school_id,
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

pub async fn get_google_client_scopes(ctx: &mut Ctx) -> Result<Scopes, sqlx::Error> {
    let get_scopes_query = prepared_query!(
        r"
            SELECT scopes
            FROM clients
            WHERE is_google = true;
        ";
        { scopes: String };
    );

    
    let res = get_scopes_query.fetch_one(&mut **ctx).await?;

    let Some(scopes) = Scopes::try_from_str(&res.scopes) else {
        return Err(sqlx::Error::Decode(Box::new(sqlx::error::Error::Protocol(
            format!("Invalid format for `Scopes` in database: {}", res.scopes),
        ))));
    };

    Ok(scopes)
}
