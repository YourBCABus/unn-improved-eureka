#![allow(unused_braces)]

use async_graphql::Object;

use crate::types::Privileges;


#[Object]
impl Privileges {
    async fn admin(&self) -> bool {
        self.admin
    }
    async fn secretary(&self) -> bool {
        self.secretary
    }
}

