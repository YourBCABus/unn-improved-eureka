use sqlx::query_as;
use uuid::Uuid;

use crate::database::Ctx;

pub struct Privileges {
    pub secretary: bool,
    pub admin: bool,
}

pub async fn get_privileges(ctx: &mut Ctx, id: Uuid) -> Result<Privileges, sqlx::Error> {
    let privileges_query = query_as!(
        Privileges,
        r#"
            SELECT
                secretary,
                admin
            FROM privileges
            WHERE teacher_id = $1;
        "#,
        id,
    );

    let privileges = privileges_query.fetch_optional(&mut **ctx).await?;
    Ok(privileges.unwrap_or(Privileges { secretary: false, admin: false }))
}
