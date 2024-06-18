use jsonwebtoken::jwk::JwkSet;

use crate::database::{self, Ctx};
use crate::logging::*;

use super::scopes::Scopes;
use super::IdTokenHeader;


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


    let mut validation = jsonwebtoken::Validation::new(header.alg);
    validation.validate_aud = false;
    validation.validate_nbf = true;
    match jsonwebtoken::decode(
        token,
        &decoding_key,
        &validation,
    ) {
        Ok(v) => {
            Ok(v.claims)
        },
        Err(e) => {
            debug!("Error kind: {e}");
            if matches!(e.kind(), jsonwebtoken::errors::ErrorKind::InvalidSignature) {
                warn!("ID token tampering detected: {e}");
            }
            Err("Failed to decode `ID-Token` header".to_string())
        }
    }
}

pub async fn user_allowed(ctx: &mut Ctx, id_token: IdTokenHeader) -> Option<Scopes> {
    let Ok(google_client_scopes) = database::prepared::clients::get_google_client_scopes(ctx).await else {
        return None;
    };
    let user_data = get_user_data(id_token).await.ok()?;

    // TODO: Remove this hardcoding
    const WHITELIST: &[&str] = &[
        "ricecrispieismyname@gmail.com",
        "skyler@rivet.gg",
    ];
    
    let is_bergen = user_data.email.ends_with("@bergen.org");
    let is_whitelisted = WHITELIST.iter().any(|&email| user_data.email == email);

    let is_allowed = is_bergen || is_whitelisted;

    if is_allowed && user_data.email_verified {
        Some(google_client_scopes)
    } else {
        Some(Scopes::new())
    }
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
