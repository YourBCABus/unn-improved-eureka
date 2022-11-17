//! This module contains some utility macros. All custom macros in this crate should be defined here,
//! but are encouraged to be reexported in other preludes.

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

macro_rules! wvn {
    ($var:ident) => {
        ($var, stringify!($var))
    };
}
pub(crate) use wvn as wvn;

macro_rules! handle_prepared {
    ($($query_names:ident),+; $mapper_fn:expr) => {
        {
            #[allow(unused_parens)]
            let output = if let ($(Some($query_names)),+) = ($($query_names),+) {
                Ok(($($query_names),+))
            } else {
                let failed_queries = [$(wvn!($query_names)),+]
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

macro_rules! build_error {
    ($reason:literal; $cause:literal => { $($other_keys:literal: $other_values:expr,)* }) => {
        {

            #[allow(clippy::match_single_binding)]
            let err_value = build_error! { impl bindings base_1_0; $cause; $($other_keys: $other_values,)*; };
            
            juniper::FieldError::new(
                $crate::graphql::prelude::utility_fns::get_dsv_cloned($reason),
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
                    build_error! { impl bindings [<$curr_name 0>]; $cause; $($other_keys: $other_values,)*; $key: [<$curr_name 0>], $($proccessed_keys: $bindings,)*}
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

macro_rules! make_id_wrapper {
    (
        $(#[$attr:meta])*
        $qual:vis $struct_name:ident
    ) => {
        
        $(#[$attr])*
        #[derive(juniper::GraphQLScalarValue, Debug, Clone)]
        $qual struct $struct_name(String);

        impl $struct_name {
            /// Create a new wrapper struct from a [Uuid]. Only create if the Uuid came from a valid teacher ID.
            pub fn new(id: &uuid::Uuid) -> Self {
                Self(id.to_string())
            }

            /// Returns a borrowed `&'a str` view of the struct. The str will only live as long as the struct. 
            pub fn id_str(&self) -> &str {
                &self.0
            }

            /// Given that the internal id string came from a valid Uuid, this function should NEVER fail.
            pub fn uuid(&self) -> uuid::Uuid {
                uuid::Uuid::try_from(self.id_str()).unwrap_or_default()
            }

            /// Will return an Ok(Uuid) if valid, otherwise returns Err(String).
            pub fn try_into_uuid(self) -> Result<(uuid::Uuid, Self), String> {
                match uuid::Uuid::try_from(self.id_str()) {
                    Ok(uuid) => Ok((uuid, self)),
                    Err(_) => Err(self.into_string())
                }
            }

            /// Get the string inside of the struct.
            /// Consumes the wrapper, and requires no references existing to the struct.
            /// Use the method `clone_to_string` to get a string without consuming the struct.
            pub fn into_string(self) -> String {
                self.0
            }

            /// Get the string inside of the wrapper struct.
            /// This method should only be used when `into_string` can't be.
            pub fn clone_to_string(&self) -> String {
                self.0.clone()
            }
        }
    };
}
pub(crate) use make_id_wrapper as make_id_wrapper;

macro_rules! make_name_wrapper {
    (
        $(#[$attr:meta])*
        $qual:vis $struct_name:ident
    ) => {
        $(#[$attr])*
        #[derive(juniper::GraphQLScalarValue, Debug, Clone)]
        $qual struct $struct_name(String);

        impl $struct_name {
            /// Create a new wrapper struct from a String. Only create if the String came from a valid student name.
            pub fn new(name: String) -> Self {
                Self(name)
            }

            /// Returns a borrowed `&'a str` view of the struct. The str will only live as long as the struct. 
            pub fn name_str(&self) -> &str {
                &self.0
            }

            /// Get the string inside of the struct.
            /// Consumes the wrapper, and requires no references existing to the struct.
            /// Use the method `clone_to_string` to get a string without consuming the struct.
            pub fn into_string(self) -> String {
                self.0
            }

            /// Get the string inside of the wrapper struct.
            /// This method should only be used when `into_string` can't be.
            pub fn clone_to_string(&self) -> String {
                self.0.clone()
            }
        }
    };
}
pub(crate) use make_name_wrapper as make_name_wrapper;

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
        impl<S: ScalarValue, $($( $lt $( : $clt $(+ $dlt )* )? ),+)?> IntoFieldError<S> for $name$(< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? {
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
    };
}
pub(crate) use make_static_enum_error as make_static_enum_error;
