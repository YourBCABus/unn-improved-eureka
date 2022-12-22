//! This module contains all the functions and methods *specifically* required for rolling HMAC verification.
//! Other verification methods should go in other modules.

use guard::guard;
use uuid::Uuid;
use warp::{Filter, hyper::body::Bytes, Rejection};

use crate::utils::str_to_uuid;


use super::{
    Payload, Signature, Secret,
    secret_map,
};

use secret_map::SecretMap;

/// This function takes all the required data for a request and client, and returns whether it passed HMAC verification.
pub fn verify_payload(payload: &Payload, signature: &Signature, counter: &[u8], secret: &Secret) -> bool {
    use hmac::{ Mac, Hmac };
    use sha2::Sha256;

    let looping_counter = counter.iter().copied().cycle();
    let acting_secret: Vec<_> = secret
        .slice().iter().cloned()
        .zip(looping_counter)
        .map(|(raw, xor)| raw ^ xor)
        .collect();

    guard!(
        let Ok(mac) = Hmac::<Sha256>::new_from_slice(&acting_secret) else { return false; }
    );
    
    let mac = mac.chain_update(payload.slice());
    
    mac.verify_slice(signature.slice()).is_ok()
}



/// This struct represents the info from the request that is required to verify said request.
/// The client ID is not included in the struct, and neither is the client secret.
struct HmacInfo {
    /// This is a `Payload` struct, which contains the bytes of the request body.
    pub body: Payload,
    /// This is a signature, passed from the header, which is the digest that the client output, and the server expects.
    pub signature: Signature,
    /// This is the counter which allows each request to have a different signature each time, without the opportunity to repeat it.
    /// This is used so even if the request is intercepted, it is invalid if recieved multiple times.
    pub counter: Vec<u8>
}

/// This function is a utility function to condense the optional inputs and string-based uuid
/// into a single Option containing the client UUID and the other HMAC info.
fn hmac_map_header_mapper(body: &[u8], signature: Option<String>, counter: Option<String>, client_id: Option<String>) -> Option<(Uuid, HmacInfo)> {
    Some((
        str_to_uuid(&client_id?).ok()?,
        HmacInfo {
            body: Payload::new(body.to_vec()),
            signature: Signature::new(hex::decode(signature?).ok()?),
            counter: hex::decode(counter?).ok()?,
        },
    ))
}


lazy_static::lazy_static! {
    static ref SECRET_MAP: SecretMap = SecretMap::uninit();
}

/// Returns a filter that checks the ROLLING HMAC validity of the request ONLY, returning true if the request passes and false otherwise.
pub fn hmac_verify_filter() -> impl Filter<Extract = ((bool, Bytes),), Error = Rejection> + Clone {
    warp::body::bytes()
        .and(warp::header::optional("hmac-signature"))
        .and(warp::header::optional("hmac-counter"))
        .and(warp::header::optional("hmac-client-id"))
        .map(|body: Bytes, signature, counter, client_id| (
            hmac_map_header_mapper(&body[..], signature, counter, client_id),
            body,
        ))
        .untuple_one()
        .map(
            |optional: Option<(Uuid, HmacInfo)>, body| (
                if let Some((client_id, hmac_info)) = optional {
                    if let Some(client_secret) = SECRET_MAP.get(&client_id) {
                        verify_payload(&hmac_info.body, &hmac_info.signature, &hmac_info.counter, &client_secret)
                    } else {
                        false
                    }
                } else {
                    false
                },
                body
            )
        )
}
