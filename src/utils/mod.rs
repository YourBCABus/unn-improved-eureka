//! Contains the utility modules [macros] and [structs],
//! along with some other often-used generic functions
//! that don't completely fit within one of the crate's submodules.

pub mod macros;

pub mod structs;


use juniper::{ ScalarValue, Value as JuniperValue };

/// A simple utility function that turns a list of "scalar-compatible" values into a GraphQL Value list.
pub fn list_to_value<OrigType, OwnedType, ScalarType>(list: Vec<&OrigType>) -> JuniperValue<ScalarType>
where
    OrigType: ToOwned<Owned = OwnedType> + ?Sized,
	OwnedType: Into<ScalarType>,
    ScalarType: ScalarValue,
{
    let value_list = list
        .into_iter()
        .map(|value| ToOwned::to_owned(value))
        .map(juniper::Value::scalar).collect();
    JuniperValue::list(value_list)
}

use uuid::{Uuid, Error as UuidError};

/// Semi-redudndant utility to get around inferred type weirdness.
pub fn str_to_uuid(string: &str) -> Result<Uuid, UuidError> {
    Uuid::try_parse(string)
}


use warp::{Rejection, hyper::body::Bytes, Filter};

/// Utility function for using the request body as both a ref, and later as an owned type.
/// Mainly useful for utilizing verification, and _then_ deserializing.
pub fn body_ref_own<'a, TR: Send + Sync, TO: Send + Sync>(
    ref_fn: impl Fn(&[u8]) -> Result<TR, Rejection> + Clone + Sync + Send + 'a,
    own_fn: impl Fn(Bytes) -> Result<TO, Rejection> + Clone + Sync + Send + 'a,
) -> impl Filter<Extract = ((TR,TO),), Error = Rejection> + Clone + 'a {
    warp::any()
        .and(warp::body::bytes())
        .map(move |bytes: Bytes| (ref_fn(&bytes[..]), bytes))
        .map(move |(ref_output, bytes)| Ok((ref_output?, own_fn(bytes)?)))
        .and_then(|result: Result<(TR, TO), Rejection>| async { result })
}
