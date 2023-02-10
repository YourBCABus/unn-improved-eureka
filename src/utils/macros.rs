//! This module contains some utility macros. All custom macros in this crate should be defined here,
//! but are encouraged to be reexported in other preludes.

/// `lazy_memoize_query` expands into a local block with a memoized variable.
/// It's mostly suggested to be used with [define_shared_query_name], to create a globally-memoized query.
macro_rules! lazy_memoize_query {
    ($query_str:literal -> $client:expr) => {
        {
            use async_once_cell::OnceCell;
            static MEMOIZED_QUERY: OnceCell<tokio_postgres::Statement> = OnceCell::new();
            MEMOIZED_QUERY.get_or_try_init(async { ($client).prepare($query_str).await })
        }
    };
}
pub(crate) use lazy_memoize_query as lazy_memoize_query;

/// Creates a function that returns a memoized query for `tokio-postgres`.
/// See [lazy_memoize_query] for more details.
macro_rules! define_shared_query_name {
    ($(#[$attr:meta])* $qual:vis $shared_name:ident: $query_str:literal) => {
        paste::paste!{
            #[doc = "A memoized referenced to the `" $shared_name "`."] 
            $qual async fn $shared_name(client: &tokio_postgres::Client) -> Option<&'static tokio_postgres::Statement> {
                $crate::preludes::macros::lazy_memoize_query!($query_str -> client).await.ok()
            }
        }
    };
}
pub(crate) use define_shared_query_name as define_shared_query_name;

/// Stands for "with variable name".
/// Returns a tuple of the variable's value and its stringified name.
macro_rules! wvn {
    ($var:ident) => {
        ($var, stringify!($var))
    };
}
pub(crate) use wvn as wvn;

/// Returns a result representing the optional query values for the given memoized function return values.
/// 
/// The error is a Vec of failed memoized prepared query names,
/// and is mapped through `$mapper_fn` before being returned as an `Err()`.
macro_rules! handle_prepared {
    ($($query_names:ident),+; $mapper_fn:expr) => {
        {
            #[allow(unused_parens)]
            let output = if let ($(Some($query_names)),+) = ($($query_names),+) {
                Ok(($($query_names),+))
            } else {
                let failed_queries = [$($crate::utils::macros::wvn!($query_names)),+]
                    .into_iter()
                    .flat_map(
                        |(option, name)| option
                            .is_none()
                            .then_some(name)
                    )
                    .collect();
                Err($mapper_fn(failed_queries))
            };
            output
        }
    };
}
pub(crate) use handle_prepared as handle_prepared;


/// A quick analog [juniper::FieldError] builder, specifically tailored to the format of errors returned by `improved-eureka`.
/// First is the "reason" for the GraphQL error, next is what will be under the `cause` field of the error extensions.
/// 
/// Finally, you can define a list of `<literal>: <expr>,` pairs.
/// 
/// The above is a convience thing solely to account for the fact that
/// juniper's [graphql_value][juniper::graphql_value] macro
/// doesn't allow expressions directly as values.
macro_rules! build_error_value {
    ($cause:literal => { $($other_keys:literal: $other_values:expr,)* }) => {
        {
            use juniper::graphql_value;
            let err_value = $crate::utils::macros::build_error_value! { impl bindings base_1_0; $cause; $($other_keys: $other_values,)*; };
            
            err_value
        }
    };

    (impl bindings $curr_name:ident; $cause:literal; ; $($proccessed_keys:literal: $bindings:ident,)*) => {
        juniper::graphql_value!({
            "cause": $cause,
            $($proccessed_keys: $bindings),*
        })
    };

    (impl bindings
        $curr_name:ident; $cause:literal;
        $key:literal: $value:expr, $($other_keys:literal: $other_values:expr,)*;
        $($proccessed_keys:literal: $bindings:ident,)*
    ) => {
            paste::paste! {
                {
                    let [<$curr_name 0>] = $value;
                    $crate::utils::macros::build_error_value! { impl bindings [<$curr_name 0>]; $cause; $($other_keys: $other_values,)*; $key: [<$curr_name 0>], $($proccessed_keys: $bindings,)*}
                }
            }
    };

    (impl val_gen $cause:literal; $curr_name:ident; ; $($other_keys:literal: $other_vals:ident,)*) => {
        juniper::graphql_value!({
            "cause": $cause,
            $($other_keys: $other_vals),*
        })
    };

    (impl val_gen $cause:literal; $curr_name:ident; $key:literal, $($unprocessed_keys:literal,)*; $($processed_keys:literal: $processed_vals:ident,)*) => {
        paste::paste! {
            build_error_value! { impl val_gen $cause; [<$curr_name 0>]; $($unprocessed_keys,)*; $($processed_keys: $processed_vals,)* $key: [<$curr_name 0>], }
        }
    };
}
pub(crate) use build_error_value as build_error_value;
/// A quick analog [juniper::FieldError] builder, specifically tailored to the format of errors returned by `improved-eureka`.
/// First is the "reason" for the GraphQL error, next is what will be under the `cause` field of the error extensions.
/// 
/// Finally, you can define a list of `<literal>: <expr>,` pairs.
/// 
/// The above is a convience thing solely to account for the fact that
/// juniper's [graphql_value][juniper::graphql_value] macro
/// doesn't allow expressions directly as values.
macro_rules! build_error {
    ($reason:literal; $cause:literal => { $($other_keys:literal: $other_values:expr,)* }) => {
        {
            use juniper::graphql_value;
            let err_value = $crate::utils::macros::build_error_value!($cause => { $($other_keys: $other_values,)* });
            
            juniper::FieldError::new(
                $crate::graphql::prelude::get_dsv_cloned($reason),
                err_value
            )
        }
    };

    (impl bindings $curr_name:ident; $cause:literal; ; $($proccessed_keys:literal: $bindings:ident,)*) => {
        juniper::graphql_value!({
            "cause": $cause,
            $($proccessed_keys: $bindings),*
        })
    };

    (impl bindings
        $curr_name:ident; $cause:literal;
        $key:literal: $value:expr, $($other_keys:literal: $other_values:expr,)*;
        $($proccessed_keys:literal: $bindings:ident,)*
    ) => {
            paste::paste! {
                {
                    let [<$curr_name 0>] = $value;
                    $crate::utils::macros::build_error! { impl bindings [<$curr_name 0>]; $cause; $($other_keys: $other_values,)*; $key: [<$curr_name 0>], $($proccessed_keys: $bindings,)*}
                }
            }
    };

    (impl val_gen $cause:literal; $curr_name:ident; ; $($other_keys:literal: $other_vals:ident,)*) => {
        juniper::graphql_value!({
            "cause": $cause,
            $($other_keys: $other_vals),*
        })
    };

    (impl val_gen $cause:literal; $curr_name:ident; $key:literal, $($unprocessed_keys:literal,)*; $($processed_keys:literal: $processed_vals:ident,)*) => {
        paste::paste! {
            build_error! { impl val_gen $cause; [<$curr_name 0>]; $($unprocessed_keys,)*; $($processed_keys: $processed_vals,)* $key: [<$curr_name 0>], }
        }
    };
}
pub(crate) use build_error as build_error;

/// A macro to build a simple byte wrapping struct.
/// It creates a new struct,
/// automatically creating `new`, `slice`, and `mut_slice` methods,
/// and also autoimplementing [Debug][std::fmt::Debug].
macro_rules! byte_vec_wrapper {
    ($(#[$attr:meta])* $name:ident) => {
        $(#[$attr])*
        pub struct $name(Vec<u8>);

        paste::paste!{
            impl $name {
                #[doc = "Creates a new instance of the `" $name "` struct."]
                pub fn new(data: Vec<u8>) -> Self {
                    Self(data)
                }

                /// Borrows a read-only slice of the contained data.
                pub fn slice(&self) -> &[u8] {
                    &self.0
                }

                /// Borrows a mutable slice of the contained data.
                pub fn mut_slice(&mut self) -> &mut [u8] {
                    &mut self.0
                }
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "Secret {{\n")?;
                for chunk in self.slice().chunks(16) {
                    for sub_chunk in chunk.chunks(2) {
                        write!(f, "{:02x}{:02x} ", sub_chunk[0], sub_chunk[1])?;
                    }
                    write!(f, "\n")?;
                }
                write!(f, "}}")
            }
        }
    };
}
pub(crate) use byte_vec_wrapper as byte_vec_wrapper;





/// A macro used to make an error enum with unit fields, where each field is associated with a `&'static str`.
/// 
/// The enum has an associated function "error_str", which returns the string after the `<variant> =>`.
macro_rules! make_unit_enum_error {
    (
        $(#[$attr:meta])*
        $qual:vis $name:ident
        $($variant:ident => $mapped_val:literal)*
    ) => {
        paste::paste! {
            $(#[$attr])*
            #[derive(Debug, Clone, Copy)]
            $qual enum $name {
                $(
                    #[doc = "Signifies failure on the `" $variant "` step."]
                    $variant,
                
                )*
            }

            impl $name {
                fn error_str(self) -> &'static str {
                    use $name::*;
                    match self {
                        $($variant => $mapped_val),*
                    }
                }
            }
        }
    };
}
pub(crate) use make_unit_enum_error as make_unit_enum_error;

/// A macro used to make an error enum to represent a SQL value.
/// This probably should be improved in the future, but...
/// TODO: MANY TO MANY FOR PERIODS.
macro_rules! make_sql_enum {
    (
        $(#[$attr:meta])*
        $qual:vis $name:ident
        $($variant:ident => $mapped_val:literal)*
    ) => {
        paste::paste! {
            $(#[$attr])*
            #[derive(Debug, Clone, Copy)]
            $qual enum $name {
                $(
                    #[doc = "The `" $variant "` state of the SQL enum."]
                    $variant,
                
                )*
            }

            impl $name {
                /// Turn the Rust enum
                pub fn to_sql_type(self) -> &'static str {
                    use $name::*;
                    match self {
                        $($variant => $mapped_val),*
                    }
                }
                pub fn try_from_sql_type(name: &str) -> Result<Self, String> {
                    use $name::*;
                    match name {
                        $($mapped_val => Ok($variant),)*
                        _ => Err(format!("Unknown variant `{}`", name)),
                    }
                }

                pub fn get_possibility_list() -> &'static [&'static str] {
                    &[ $($mapped_val),* ]
                }

                pub fn get_sql_typedef() -> String {
                    let mut out_string = String::default();
                    out_string.push_str("ENUM (");

                    $(out_string.push_str(concat!("'", $mapped_val, "', "));)*

                    out_string.pop();
                    out_string.pop();
                    out_string.push_str(")");

                    out_string
                }
            }
        }
    };
}
pub(crate) use make_sql_enum as make_sql_enum;

/// This is a really powerful utility macro to make an enum which can directly
/// represent and format an error that will be returned to the client.
/// 
/// This can represent both client errors and server errors in the SAME enum.
/// FIXME: This documentation NEEDS to be improved. An example should be provided.
macro_rules! make_static_enum_error {
    (
        $($scalar_value_param:ident;;)?
        $(#[$attr:meta])+
        $qual:vis $name:ident $(< $( $lt:ty $( : $clt:ty )? ),+ >)?;
        $($(#[$variant_attr:meta])+ $variant:ident ( $($member:ty),* )
            => $reason:literal,
                $cause:literal ==> |$($closure_param:ident),*| {
                    $($other_keys:literal: $other_values:expr,)*
                };
        )*
    ) => {
            make_static_enum_error!{ 
                @impl struct_def
                $($scalar_value_param)?
                $(#[$attr])+
                $qual $name $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)?;
                $($(#[$variant_attr])+ $variant ( $($member),* ))*
            }

            make_static_enum_error!{
                @impl impl_block
                $($scalar_value_param)?
                $name $(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)?;
                $(
                    $variant
                        => $reason,
                            $cause ==> |$($closure_param),*| {
                                $($other_keys: $other_values,)*
                            };
                )*
            }
    };
    (@impl struct_def
        $scalar_value_param:ident
        $(#[$attr:meta])+
        $qual:vis $name:ident $(< $( $lt:ty $( : $clt:ty )? ),+ >)?;
        $($(#[$variant_attr:meta])+ $variant:ident ( $($member:ty),* ))*
    ) => {
        
        paste::paste! {
            $(#[$attr])+
            #[derive(Debug, Clone)]
            $qual enum $name<$scalar_value_param: ScalarValue, $($( $lt $( : $clt $(+ $dlt )* )? ),+)? > {
                $(
                    $(#[$variant_attr])+
                    $variant ( $($member),* ),
                )*
            }
        }
    };
    (@impl struct_def
        $(#[$attr:meta])+
        $qual:vis $name:ident $(< $( $lt:ty $( : $clt:ty )? ),+ >)?;
        $($(#[$variant_attr:meta])+ $variant:ident ( $($member:ty),* ))*
    ) => {
        
        paste::paste! {
            $(#[$attr])+
            #[derive(Debug, Clone)]
            $qual enum $name$(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? {
                $(
                    $(#[$variant_attr])+
                    $variant ( $($member),* ),
                )*
            }
        }
    };
    (@impl impl_block
        $scalar_value_param:ident
        $name:ident $(< $( $lt:ty $( : $clt:ty )? ),+ >)?;
        $(
            $variant:ident
                => $reason:literal,
                    $cause:literal ==> |$($closure_param:ident),*| {
                        $($other_keys:literal: $other_values:expr,)*
                    };
        )*
    ) => {
        impl<$scalar_value_param: ScalarValue, $($( $lt $( : $clt $(+ $dlt )* )? ),+)?> IntoFieldError<$scalar_value_param> for $name<$scalar_value_param, $($($lt),+)?> {
            fn into_field_error(self) -> FieldError<S> {
                use $name::*;
        
                match self {
                    $(
                        $variant ($($closure_param),*) => $crate::utils::macros::build_error!(
                            $reason; $cause => {
                                $($other_keys: $other_values,)*
                            }
                        )
                    ),*
                    
                }
            }
        }
        impl<$scalar_value_param: ScalarValue, $($( $lt $( : $clt $(+ $dlt )* )? ),+)?> From<$name<$scalar_value_param, $($($lt),+)?>> for juniper::Value<$scalar_value_param> {
            fn from(self_enum: $name<$scalar_value_param, $($($lt),+)?>) -> Self {
                use $name::*;
        
                match self_enum {
                    $(
                        $variant ($($closure_param),*) => $crate::utils::macros::build_error_value!(
                            $cause => {
                                $($other_keys: $other_values,)*
                            }
                        )
                    ),*
                    
                }
            }
        }
    };
    (@impl impl_block
        $name:ident $(< $( $lt:ty $( : $clt:ty )? ),+ >)?;
        $(
            $variant:ident
                => $reason:literal,
                    $cause:literal ==> |$($closure_param:ident),*| {
                        $($other_keys:literal: $other_values:expr,)*
                    };
        )*
    ) => {
        impl<S: juniper::ScalarValue, $($( $lt $( : $clt $(+ $dlt )* )? ),+)?> juniper::IntoFieldError<S> for $name$(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? {
            fn into_field_error(self) -> juniper::FieldError<S> {
                use $name::*;
        
                match self {
                    $(
                        $variant ($($closure_param),*) => $crate::utils::macros::build_error!(
                            $reason; $cause => {
                                $($other_keys: $other_values,)*
                            }
                        )
                    ),*
                    
                }
            }
        }
        impl<S: juniper::ScalarValue, $($( $lt $( : $clt $(+ $dlt )* )? ),+)?> From<$name$(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)?> for juniper::Value<S> {
            fn from(self_enum: $name$(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)?) -> Self {
                use $name::*;
        
                match self_enum {
                    $(
                        $variant ($($closure_param),*) => $crate::utils::macros::build_error_value!(
                            $cause => {
                                $($other_keys: $other_values,)*
                            }
                        )
                    ),*
                    
                }
            }
        }
    };
}
pub(crate) use make_static_enum_error as make_static_enum_error;
