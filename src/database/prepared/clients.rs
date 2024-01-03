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

pub struct SheetId { id: String }
pub async fn get_sheet_id(ctx: &mut Ctx) -> Result<String, sqlx::Error> {
    let get_key_query = query_as!(
        SheetId,
        r#"
            SELECT sheet_id AS id
            FROM config;
        "#,
    );

    let res = get_key_query.fetch_one(&mut **ctx).await?;

    Ok(res.id)
}

pub async fn set_sheet_id(ctx: &mut Ctx, id: &str) -> Result<(), sqlx::Error> {
    let set_key_query = query_as!(
        SheetId,
        r#"
            UPDATE config
            SET sheet_id = $1;
        "#,
        id,
    );

    set_key_query.execute(&mut **ctx).await?;

    Ok(())
}
