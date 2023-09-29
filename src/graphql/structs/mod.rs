//! This module contains most of the structs compatible with graphql query/mutation resolution.

pub mod inputs;

pub use inputs::*;


// pub mod juniper_types {
//     //! Contains some junipers types that are useful to have quick access to.
//     pub use juniper::IntoFieldError;
//     pub use juniper::ScalarValue;
//     pub use juniper::Value as JuniperValue;

// }

// // pub mod teachers {
// //     pub use super::scalars::teacher::*;
// //     pub use super::objects::teacher::*;
// // }

// /// A macro to build a Uuid-wrapping struct.
// /// 
// /// Internally, it is a String, but with the `try_into_uuid` function, it will either return either
// /// - an Ok containing tuple (Uuid, IdWrapper)
// /// - or Err containing the internal String
// macro_rules! make_id_wrapper {
//     (
//         $(#[$attr:meta])*
//         $qual:vis $struct_name:ident
//     ) => {
        
//         $(#[$attr])*
//         #[derive(juniper::GraphQLScalarValue, Debug, Clone)]
//         $qual struct $struct_name(String);

//         impl $struct_name {
//             /// Create a new wrapper struct from a [Uuid]. Only create if the Uuid came from a valid teacher ID.
//             pub fn new(id: &uuid::Uuid) -> Self {
//                 Self(id.to_string())
//             }

//             /// Returns a borrowed `&'a str` view of the struct. The str will only live as long as the struct. 
//             pub fn id_str(&self) -> &str {
//                 &self.0
//             }

//             /// Given that the internal id string came from a valid Uuid, this function should NEVER fail.
//             pub fn uuid(&self) -> uuid::Uuid {
//                 self.try_uuid().unwrap_or_default()
//             }

//             /// Will return an Ok((Uuid, Self)) if valid, otherwise returns Err(String).
//             pub fn try_uuid(&self) -> Result<uuid::Uuid, String> {
//                 uuid::Uuid::try_from(self.id_str()).map_err(|_| self.clone_to_string())
//             }

//             /// Will return an Ok((Uuid, Self)) if valid, otherwise returns Err(String).
//             pub fn try_into_uuid(self) -> Result<(uuid::Uuid, Self), Self> {
//                 match uuid::Uuid::try_from(self.id_str()) {
//                     Ok(uuid) => Ok((uuid, self)),
//                     Err(_) => Err(self)
//                 }
//             }

//             /// Get the string inside of the struct.
//             /// Consumes the wrapper, and requires no references existing to the struct.
//             /// Use the method `clone_to_string` to get a string without consuming the struct.
//             pub fn into_string(self) -> String {
//                 self.0
//             }

//             /// Get the string inside of the wrapper struct.
//             /// This method should only be used when `into_string` can't be.
//             pub fn clone_to_string(&self) -> String {
//                 self.0.clone()
//             }
//         }
//     };
// }
// pub(crate) use make_id_wrapper as make_id_wrapper;

// /// A macro to build a name-wrapping struct.
// /// 
// /// Internally, the wrapper contains an owned String and you can
// /// - access an immutable &str with `name_str`
// /// - consume the wrapper and get the inner string with `into_string`
// /// - get a cloned version of the string from just a reference with `clone_to_string`
// macro_rules! make_name_wrapper {
//     (
//         $(#[$attr:meta])*
//         $qual:vis $struct_name:ident
//     ) => {
//         $(#[$attr])*
//         #[derive(juniper::GraphQLScalarValue, Debug, Clone)]
//         $qual struct $struct_name(String);

//         impl $struct_name {
//             /// Create a new wrapper struct from a String. Only create if the String came from a valid student name.
//             pub fn new(name: String) -> Self {
//                 Self(name)
//             }

//             /// Returns a borrowed `&'a str` view of the struct. The str will only live as long as the struct. 
//             pub fn name_str(&self) -> &str {
//                 &self.0
//             }

//             /// Get the string inside of the struct.
//             /// Consumes the wrapper, and requires no references existing to the struct.
//             /// Use the method `clone_to_string` to get a string without consuming the struct.
//             pub fn into_string(self) -> String {
//                 self.0
//             }

//             /// Get the string inside of the wrapper struct.
//             /// This method should only be used when `into_string` can't be.
//             pub fn clone_to_string(&self) -> String {
//                 self.0.clone()
//             }
//         }
//     };
// }
// pub(crate) use make_name_wrapper as make_name_wrapper;
