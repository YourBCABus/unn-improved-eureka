#![allow(unused_braces)]

use async_graphql::Object;
use crate::types::TeacherName;

#[Object]
impl TeacherName {
    async fn honorific(&self) -> &str { self.get_honorific().str() }
    
    async fn first(&self) -> &str { self.get_first() }
    async fn middles(&self) -> Vec<&str> { self.visible_middles().collect() }
    async fn last(&self) -> &str { self.get_last() }


    async fn full(&self) -> String { self.longest() }
    async fn first_last(&self) -> String { self.mid_len() }
    async fn normal(&self) -> String { self.short() }
}
