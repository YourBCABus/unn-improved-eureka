use uuid::Uuid;

use crate::database::{self, Ctx};

use super::scopes::Scopes;

pub async fn client_allowed(client_id: Uuid, provided_secret: &[u8], ctx: &mut Ctx) -> Option<Scopes> {
    let Ok(Some(secret)) = database::prepared::clients::get_client_secret(ctx, client_id).await else {
        return None;
    };

    let (hash, salt) = secret.rsplit_once(':')?;

    let value_to_hash: Vec<u8> = provided_secret
        .iter()
        .copied()
        .chain(std::iter::once(b':'))
        .chain(salt.as_bytes().iter().copied())
        .collect();


    let test_hash = sha256::digest(value_to_hash);

    if !constant_time_eq::constant_time_eq(test_hash.as_bytes(), hash.as_bytes()) {
        return None;
    }

    database::prepared::clients::get_client_scopes(ctx, client_id).await.ok().flatten()
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
