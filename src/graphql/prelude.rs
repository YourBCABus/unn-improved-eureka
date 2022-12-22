//! A collection of things generally needed when dealing with implementing/using a GraphQL query or mutation.
//! 
//! Either this or [crate::preludes::graphql] are the recommended way to get access to the most important GraphQL items.
//! 
//! If you don't want to pollute your namespace, you still should be able to find this useful with 
//! `use <this module>::{ item1, item_2 };`

use std::sync::Arc;

pub use crate::preludes::*;

pub use tokio_postgres::Client;

pub use juniper::{
    EmptySubscription as NoSubscription,
    DefaultScalarValue as DSV,
    RootNode as GraphQLRoot,

    FieldError,
    IntoFieldError,
    ScalarValue,
    graphql_value,
};

pub use juniper::http::{
    GraphQLRequest,
    graphiql::graphiql_source,
};

pub use {
    super::{
        Schema,
        mutations::MutationRoot,
        queries::QueryRoot,
        Context,
    },
    super::{
        exec_graphql,
    },

    super::structs::*,
};

/// General utility functions for graphql-related things.
/// 
/// All of these functions are in the general scope of [the prelude][self],
/// so it shouldn't be necessary to access this (hence the lack of `pub`).
mod utility_fns {
    use super::*;

    /// Build a generic schema based on the types defined in the [graphql] module.
    /// 
    /// The print attribute will determine whether the schema should be printed to stdout.
    /// 
    /// TODO: Allow output to file location
    pub fn easy_build_schema(print: bool) -> Arc<Schema> {
        let schema = Arc::new(Schema::new(QueryRoot, MutationRoot, NoSubscription::new()));

        if print {
            println!("--------SCHEMA--------\n\n{}\n\n--------SCHEMA--------", schema.as_schema_language());
        }

        schema
    }
    
    /// This is just a conversion helper function due to the weirdness around juniper's/graphql's "Scalar Value".
    pub fn get_dsv<T>(value: T) -> DSV
    where
        DSV: From<T>
        {
        DSV::from(value)
    }

    /// Similar to [get_dsv], but it instead allows types implementing [ToOwned] to be similarly wrapped as owned types.
    pub fn get_dsv_cloned<T>(value: &T) -> DSV
    where
        T: ToOwned + ?Sized,
        DSV: From<T::Owned>
        {
        DSV::from(value.to_owned())
    }
}

pub use utility_fns::*;


