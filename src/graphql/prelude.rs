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

pub mod utility_fns {
    use super::*;

    pub fn easy_build_schema() -> Arc<Schema> {
        Arc::new(Schema::new(QueryRoot, MutationRoot, NoSubscription::new()))
    }
    
    // pub fn get_dsv<T: From<>>(value: )
    pub fn get_dsv<T>(value: T) -> DSV
    where
        DSV: From<T>
        {
        DSV::from(value)
    }
    
    pub fn get_dsv_cloned<T>(value: T) -> DSV
    where
        T: ToOwned,
        DSV: From<T::Owned>
        {
        #[allow(clippy::redundant_clone)] // This isn't redundant, need to figure out why clippy is complaining.
        DSV::from(value.to_owned())
    }
}

pub use utility_fns::*;


