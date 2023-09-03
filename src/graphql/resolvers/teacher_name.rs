#![allow(unused_braces)]

use crate::types::TeacherName;
use crate::state::AppState;

use juniper::graphql_object;


#[graphql_object(
    context = AppState,
    name = "TeacherName",
)]
impl TeacherName {
    fn honorific(&self) -> &str { self.honorific() }
    
    fn first(&self) -> &str { self.first() }
    fn middles(&self) -> Vec<&str> { self.visible_middles().collect() }
    fn last(&self) -> &str { self.last() }


    fn full(&self) -> &str { &self.longest() }
    fn first_last(&self) -> &str { &self.mid_len() }
    fn normal(&self) -> &str { &self.short() }
}
