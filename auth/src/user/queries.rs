use regex::Regex;
use uuid::Uuid;

use logging::*;

use db::Ctx;
use db::prepared_query;

use crate::scopes::Scopes;

pub async fn get_google_scopes(ctx: &mut Ctx, school_id: Uuid) -> Result<Scopes, sqlx::Error> {
    let get_scopes_query = prepared_query!(
        r"
            SELECT scopes
            FROM clients
            WHERE
                is_google AND
                school_id = $1;
        ";
        { scopes: String };
        school_id
    );

    
    let res = get_scopes_query.fetch_one(&mut **ctx).await?;

    let Some(scopes) = Scopes::try_from_str(&res.scopes) else {
        return Err(sqlx::Error::Decode(Box::new(sqlx::error::Error::Protocol(
            format!("Invalid format for `Scopes` in database: {}", res.scopes),
        ))));
    };

    Ok(scopes)
}

pub async fn get_school_email_regexes(ctx: &mut Ctx, school_id: Uuid) -> Result<Vec<Regex>, sqlx::Error> {
    let get_regexes_query = prepared_query!(
        r"
            SELECT email_regexes
            FROM google_emails
            WHERE school_id = $1;
        ";
        { email_regexes: Vec<String> };
        school_id
    );

    
    let res = get_regexes_query.fetch_one(&mut **ctx).await?;

    let mut regexes = Vec::with_capacity(res.email_regexes.len());
    for regex in res.email_regexes {
        match Regex::new(&regex) {
            Ok(regex) => regexes.push(regex),
            Err(err) => return Err(sqlx::Error::Decode(Box::new(sqlx::error::Error::Protocol({
                debug!("Invalid regex in database: (regex {regex}), (err {err})");
                "Invalid regex in database".to_string()
            })))),
        }
    }

    Ok(regexes)
}

pub async fn get_school_hosted_domains(ctx: &mut Ctx, school_id: Uuid) -> Result<Vec<String>, sqlx::Error> {
    let get_hds_query = prepared_query!(
        r"
            SELECT hosted_domains
            FROM google_emails
            WHERE school_id = $1;
        ";
        { hosted_domains: Vec<String> };
        school_id
    );

    Ok(get_hds_query.fetch_one(&mut **ctx).await?.hosted_domains)
}
