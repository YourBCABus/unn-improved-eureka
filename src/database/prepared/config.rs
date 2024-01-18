use crate::database::Ctx;

use super::prepared_query;

/** 
 * Sheet ID
 */

mod sheet_id {
    use super::{ prepared_query, Ctx };

    pub async fn get(ctx: &mut Ctx) -> Result<String, sqlx::Error> {
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
    
    pub async fn set(ctx: &mut Ctx, id: &str) -> Result<(), sqlx::Error> {
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
}
pub use sheet_id::{ get as get_sheet_id, set as set_sheet_id };







/** 
 * Report To
 */

mod report_to {
    use super::{ prepared_query, Ctx };

    pub async fn get(ctx: &mut Ctx) -> Result<String, sqlx::Error> {
        let get_key_query = prepared_query!(
            r"
                SELECT report_to
                FROM config;
            ";
            { report_to: String };
        );
    
        let res = get_key_query.fetch_one(&mut **ctx).await?;
    
        Ok(res.report_to)
    }
    
    pub async fn set(ctx: &mut Ctx, report_to: &str) -> Result<(), sqlx::Error> {
        let set_key_query = prepared_query!(
            r"
                UPDATE config
                SET report_to = $1;
            ";
            {  };
            report_to
        );
    
        set_key_query.execute(&mut **ctx).await?;
    
        Ok(())
    }
}
pub use report_to::{ get as get_report_to, set as set_report_to };