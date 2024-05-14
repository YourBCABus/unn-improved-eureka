use crate::database::Ctx;

use super::prepared_query;

/** 
 * Sheet ID
 */

mod sheet_id {
    use uuid::Uuid;

    use super::{ prepared_query, Ctx };

    pub async fn get(ctx: &mut Ctx, school_id: Uuid) -> Result<String, sqlx::Error> {
        let get_key_query = prepared_query!(
            r"
                SELECT sheet_id
                FROM config
                WHERE school_id = $1;
            ";
            { sheet_id: String };
            school_id,
        );
    
        let res = get_key_query.fetch_one(&mut **ctx).await?;
    
        Ok(res.sheet_id)
    }
    
    pub async fn set(ctx: &mut Ctx, school_id: Uuid, id: &str) -> Result<(), sqlx::Error> {
        let set_key_query = prepared_query!(
            r"
                UPDATE config
                SET sheet_id = $1
                WHERE school_id = $2;
            ";
            {  };
            id,
            school_id,
        );
    
        set_key_query.execute(&mut **ctx).await?;
    
        Ok(())
    }
}
pub use sheet_id::{ get as get_sheet_id, set as set_sheet_id };







/** 
 * Report To
 */

mod report_to {
    use uuid::Uuid;

    use super::{ prepared_query, Ctx };

    pub async fn get(ctx: &mut Ctx, school_id: Uuid) -> Result<String, sqlx::Error> {
        let get_key_query = prepared_query!(
            r"
                SELECT report_to
                FROM config
                WHERE school_id = $1;
            ";
            { report_to: String };
            school_id,
        );
    
        let res = get_key_query.fetch_one(&mut **ctx).await?;
    
        Ok(res.report_to)
    }
    
    pub async fn set(ctx: &mut Ctx, school_id: Uuid, report_to: &str) -> Result<(), sqlx::Error> {
        let set_key_query = prepared_query!(
            r"
                UPDATE config
                SET report_to = $1
                WHERE school_id = $2;
            ";
            {  };
            report_to,
            school_id,
        );
    
        set_key_query.execute(&mut **ctx).await?;
    
        Ok(())
    }
}
pub use report_to::{ get as get_report_to, set as set_report_to };



/** 
 * Attribs
 */

mod attribs {
    use std::collections::HashMap;

    use super::{ prepared_query, Ctx };
    use sqlx::types::JsonValue;
    use uuid::Uuid;

    pub async fn get(ctx: &mut Ctx, school_id: Uuid) -> Result<HashMap<String, JsonValue>, sqlx::Error> {
        let get_key_query = prepared_query!(
            r"
                SELECT attribs
                FROM config
                WHERE school_id = $1;
            ";
            { attribs: JsonValue };
            school_id,
        );
    
        let res = get_key_query.fetch_one(&mut **ctx).await?;
        let attribs = res.attribs;

        if let JsonValue::Object(attribs) = attribs {
            Ok(attribs.into_iter().collect())
        } else {
            Err(sqlx::Error::Decode("config.attribs was not a valid JSON object".to_string().into()))
        }
    }
    
    pub async fn set_key(ctx: &mut Ctx, school_id: Uuid, key: &str, attrib: &JsonValue) -> Result<(), sqlx::Error> {
        let key = [key.to_string()];
        let set_key_query = prepared_query!(
            r"
            UPDATE config
            SET attribs = jsonb_set_lax(attribs, $1, $2, true, 'use_json_null')
            WHERE school_id = $3;
            ";
            {  };
            key.as_slice(), attrib, school_id,
        );
    
        set_key_query.execute(&mut **ctx).await?;
    
        Ok(())
    }

    pub async fn clear_key(ctx: &mut Ctx, school_id: Uuid, key: &str) -> Result<(), sqlx::Error> {
        let key = [key.to_string()];
        let set_key_query = prepared_query!(
            r"
            UPDATE config
            SET attribs = jsonb_set_lax(attribs, $1, null, true, 'delete_key')
            WHERE school_id = $2;
            ";
            {  };
            key.as_slice(),
            school_id,
        );
    
        set_key_query.execute(&mut **ctx).await?;
    
        Ok(())
    }

    pub async fn set(ctx: &mut Ctx, school_id: Uuid, attribs: HashMap<String, JsonValue>) -> Result<(), sqlx::Error> {
        let attribs = JsonValue::Object(attribs.into_iter().collect());
        let set_key_query = prepared_query!(
            r"
                UPDATE config
                SET attribs = $1
                WHERE school_id = $2;
            ";
            {  };
            attribs,
            school_id,
        );
    
        set_key_query.execute(&mut **ctx).await?;
    
        Ok(())
    }
}
pub use attribs::{
    get as get_attribs,
    set_key as set_single_attrib,
    clear_key as clear_single_attrib,
    set as set_attribs,
};


pub async fn get_default_school_id(ctx: &mut Ctx) -> Result<uuid::Uuid, sqlx::Error> {
    let get_default_school_id_query = prepared_query!(
        r"
            SELECT id
            FROM schools
            WHERE is_default = true;
        ";
        { id: uuid::Uuid };
    );

    get_default_school_id_query.fetch_one(&mut **ctx).await.map(|v| v.id)
}
