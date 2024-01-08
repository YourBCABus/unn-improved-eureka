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

pub async fn get_sheet_id(ctx: &mut Ctx) -> Result<String, sqlx::Error> {
    let get_key_query = prepared_query!(
        r"
            SELECT sheet_id
            FROM config;
        ";
        { sheet_id: String };
    );

    let res = get_key_query.fetch_one(&mut **ctx).await?;

    Ok(res.sheet_id)
}

pub async fn set_sheet_id(ctx: &mut Ctx, id: &str) -> Result<(), sqlx::Error> {
    let set_key_query = prepared_query!(
        r"
            UPDATE config
            SET sheet_id = $1;
        ";
        {  };
        id
    );

    set_key_query.execute(&mut **ctx).await?;

    Ok(())
}
