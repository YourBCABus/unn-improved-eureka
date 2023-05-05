
// use std::{fmt::Display, ops::Range};

use std::fmt::Debug;



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NameParts {
    first: String,
    last: String,
    honorific: String,
}

impl NameParts {
    pub fn new<S: ToOwned<Owned = String>>(first: S, last: S, honorific: S) -> Self {
        Self {
            first: first.to_owned(),
            last: last.to_owned(),
            honorific: honorific.to_owned(),
        }
    }
}

impl NameParts {
    pub fn first(&self) -> &str { &self.first }
    pub fn last(&self) -> &str { &self.last }
    pub fn full_name(&self) -> String { format!("{} {}", self.first, self.last) }
    pub fn full_name_honorific(&self) -> String { format!("{}. {} {}", self.honorific, self.first, self.last) }
    pub fn honorific(&self) -> &str { &self.honorific }
}


mod braces_module {
    //! It's almost disappointing that this has to exist.
    #![allow(unused_braces)]
    use super::NameParts;
    
    use juniper::graphql_object;
    use crate::graphql_types::Context;

    #[graphql_object(
        context = Context,
        name = "NameParts",
    )]
    impl NameParts {
        fn first(&self) -> &str { &self.first }
        fn last(&self) -> &str { &self.last }
        fn full_name(&self) -> String { self.full_name() }
        fn full_name_honorific(&self) -> String { self.full_name_honorific() }
        fn honorific(&self) -> &str { &self.honorific }
    }
}


