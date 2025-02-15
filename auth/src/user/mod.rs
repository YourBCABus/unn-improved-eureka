use jsonwebtoken::jwk::JwkSet;
use uuid::Uuid;

use db::Ctx;
use logging::*;

use super::scopes::Scopes;
use super::IdTokenHeader;

mod queries;

pub async fn get_google_keys() -> Result<JwkSet, String> {
    #[derive(Debug, Clone, serde::Deserialize)]
    struct WellKnownJwksUri {
        jwks_uri: String,
    }

    static JWKS: tokio::sync::OnceCell<JwkSet> = tokio::sync::OnceCell::const_new();

    JWKS.get_or_try_init(|| async {
        const OPEN_ID_WELLKNOWN_CONFIG: &str = "https://accounts.google.com/.well-known/openid-configuration";
        let openid_wellknown = reqwest::get(OPEN_ID_WELLKNOWN_CONFIG).await;
        let openid_wellknown = openid_wellknown.map_err(|e| e.to_string())?;
        let openid_wellknown: WellKnownJwksUri = openid_wellknown.json().await.map_err(|e| e.to_string())?;

        let jwks = reqwest::get(openid_wellknown.jwks_uri).await;
        let jwks = jwks.map_err(|e| e.to_string())?;
        jwks.json().await.map_err(|e| e.to_string())
    }).await.cloned()
}


#[derive(Debug, Clone, serde::Deserialize)]
struct RelevantUserInfo {
    email: String,
    email_verified: bool,
    hd: Option<String>,
}

async fn get_user_data(id_token: IdTokenHeader) -> Result<RelevantUserInfo, String> {
    let bytes = id_token.as_bytes();
    let Ok(token) = std::str::from_utf8(bytes) else {
        return Err("Invalid `Id-Token` header".to_string());
    };


    let keys = get_google_keys().await?;
    let header = jsonwebtoken::decode_header(token).map_err(|e| e.to_string())?;
    let Some(kid) = header.kid else {
        return Err("No Key ID specified".to_string());
    };
    let Some(key) = keys.find(&kid) else {
        return Err(format!("Unknown key ID {:?}", kid));
    };
    let Ok(decoding_key) = jsonwebtoken::DecodingKey::from_jwk(key) else {
        return Err("Could not convert JWK to decoding key".to_string());
    };


    // TODO: Move accepted client IDs to .env variables
    let mut validation = jsonwebtoken::Validation::new(header.alg);
    validation.validate_nbf = true;
    validation.set_audience(&[
        "272982920556-4j4j3s8t7l97q7h949gf2of71ak45hdi.apps.googleusercontent.com", // Android Prod
        "272982920556-l5jaaqqu5thbe3237io6f5o0fle95s42.apps.googleusercontent.com", // Android Dev
        "272982920556-erujjqbvuiu4880bvtg7q0vdrpc84chq.apps.googleusercontent.com", // iOS
        "272982920556-82qhftjei4mhs0sm5g91dutu655tkdd0.apps.googleusercontent.com", // Web
    ]);
    validation.set_issuer(&[
        "accounts.google.com",
        "https://accounts.google.com",
    ]);

    match jsonwebtoken::decode(
        token,
        &decoding_key,
        &validation,
    ) {
        Ok(v) => {
            Ok(v.claims)
        },
        Err(e) => {
            info!("ID token error kind: {e}");
            if matches!(e.kind(), jsonwebtoken::errors::ErrorKind::InvalidSignature) {
                warn!("ID token tampering detected: {e}");
            }
            Err("Failed to decode `ID-Token` header".to_string())
        }
    }
}

pub async fn user_allowed(ctx: &mut Ctx, school_id: Uuid, id_token: IdTokenHeader) -> Result<Scopes, String> {
    use queries::get_google_scopes as google_scopes;
    use queries::get_school_email_regexes as email_regexes;
    use queries::get_school_hosted_domains as hosted_domains;

    // Get & verify the token data, break out if email isn't verified
    let user_data = get_user_data(id_token).await?;
    if !user_data.email_verified {
        return Err("Unverified email".to_string());
    }

    // Get the google scopes to grant if the hosted domain or email matches
    let google_scopes = google_scopes(ctx, school_id).await.map_err(|v| format!("{v:?}"))?;

    // If the hosted domain is one of the accepted ones for THIS SCHOOL, return
    // the authorized scopes
    let hosted_domains = hosted_domains(ctx, school_id).await.map_err(|v| format!("{v:?}"))?;
    if let Some(domain) = &user_data.hd {
        if hosted_domains.contains(domain) {
            return Ok(google_scopes);
        }
    }

    // If the email matches any of the accepted regexes for THIS SCHOOL, return
    // the authorized scopes
    let email_regexes = email_regexes(ctx, school_id).await.map_err(|v| format!("{v:?}"))?;
    if email_regexes.iter().any(|regex| regex.is_match(&user_data.email)) {
        return Ok(google_scopes);
    }

    // Otherwise, return the default scopes
    Err("Unauthorized User".to_string())
}

pub fn generate_client_keystr(secret: &[u8]) -> Option<String> {
    use rand::RngCore;
    let mut rng = rand::thread_rng();

    let mut salt_bytes = [0; 32];
    if rng.try_fill_bytes(&mut salt_bytes).is_err() {
        return None;
    }

    let salt = hex::encode(salt_bytes);

    let value_to_hash: Vec<u8> = secret
        .iter()
        .copied()
        .chain(std::iter::once(b':'))
        .chain(salt.as_bytes().iter().copied())
        .collect();

    let hash = sha256::digest(value_to_hash);

    let keystr = format!("{hash}:{salt}");
    Some(keystr)
}
